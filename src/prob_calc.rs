use bevy::prelude::*;
use rand::Rng;
use crate::fish::*;
use crate::gameday::*;
use crate::species::*;
use crate::weather::*;

const BASE_HUNGER_PROB: f32 = 0.7;
const INC_IND: usize = 32;
const AGE_OLD: usize = 1;
const W_NOT_PREF: usize = 2;
const T_NOT_PREF: usize = 4;
const LOW_HOOK: usize = 8;
const MED_HOOK: usize = 16;
const HIGH_HOOK: usize  = 24;

#[derive(Resource)]
pub struct ProbTimer{
    pub timer: Timer,
}

impl ProbTimer {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Repeating),
        }
    }    
} 

pub fn calc_fish_prob(
    fish: &mut Fish, 
    species: &Species, 
    weather: &Res<WeatherState>, 
    time: &Res<GameDayTimer>) -> f32
    {
        let fish_hunger = fish.hunger;
        let mut a = 0.05 + (0.05*fish_hunger);
        let mut b_a = 0.;
        let mut b = 0.;
        if species.weather == weather.current_weather && (time.hour >= (species.time_of_day.0 as i32) && time.hour <= (species.time_of_day.1 as i32)) {
            b_a = species.catch_prob;
            b = (0.25)*(((species.time_of_day.1 as f32)-(species.time_of_day.0 as f32))/24.);
        }
        else if species.weather == weather.current_weather || (time.hour >= (species.time_of_day.0 as i32) && time.hour <= (species.time_of_day.1 as i32)) {
            b_a = species.catch_prob/2.;
            if species.weather == weather.current_weather {
                b = (0.25)*(1. - (((species.time_of_day.1 as f32)-(species.time_of_day.0 as f32))/24.));
            }
            else {
                b = (0.75)*(((species.time_of_day.1 as f32)-(species.time_of_day.0 as f32))/24.);
            }
        }
        else{
            b_a = species.catch_prob / 4.;
            b = (0.75)*(1. - (((species.time_of_day.1 as f32)-(species.time_of_day.0 as f32))/24.));
        }

        let mut result = (b_a*a)/b;
        println!("a = {}\nb = {}\nb_a = {}\nProb: {}", a, b, b_a, result);
        if result > 0.99 {
            result = 0.99;
        }
        
        return result;
}

pub fn hook_fish(
    mut potential_fish: (&mut Fish, &Species),
    weather: &Res<WeatherState>,
    timer: &Res<GameDayTimer>,
    mut prob_timer: &mut ResMut<ProbTimer>,
    time: &Res<Time>
    ) -> bool {

        prob_timer.timer.tick(time.delta());
        if prob_timer.timer.just_finished() {
                let (fish, species) = potential_fish;
                let prob = 100. * calc_fish_prob(fish, species, &weather, &timer);
                println!("ok");
                let mut prob_rng = rand::thread_rng();
                let roll = prob_rng.gen_range(0..100);
                println!("Prob: {}\tRoll: {}", prob, roll);
                if (roll as f32) < prob {
                    return true;
                }
            }
            return false;      
        
    }

#[derive(Component)]
pub struct HungerCpt{
    pub cpt: [f32; 64]
}

impl HungerCpt {
    pub fn new(time_pref: (usize, usize)) -> Self{
        let hrs_pref = time_pref.1 - time_pref.0 + 1;
        let mut in_cpt: [f32; 64] = [0.; 64];
        let mut ind = 0;
        //hook ct levels
        for i in 0..4 {
            let mut hook_ct_prob = 1. - (0.1*(i as f32));
            //time pref
            for j in 0..2{
                let mut time_pref_prob: f32 = 0.;
                if j == 0 {
                    time_pref_prob = Self::h_t(hrs_pref);
                }
                else{
                    time_pref_prob = Self::h_not_t(hrs_pref, Self::h_t(hrs_pref));
                }
                //weather pref
                for k in 0..2{
                    let mut weather_pref_prob: f32 = 0.;
                    if k == 0 {
                        weather_pref_prob = ((0.25*1.25)*BASE_HUNGER_PROB)/0.25;
                    }
                    else {
                        weather_pref_prob = (0.25-((0.25*1.25)*BASE_HUNGER_PROB))/0.75;
                    }
                    //age
                    for n in 0..2{
                        let mut age_prob = 0.7 + (0.3*(n as f32));
                        in_cpt[ind] = hook_ct_prob*time_pref_prob*weather_pref_prob*age_prob;
                        in_cpt[32+ind] = 1. - (hook_ct_prob*time_pref_prob*weather_pref_prob*age_prob);
                        println!("ct prob:{}  time_prob:{}  weather_prob:{}  age_prob:{}", hook_ct_prob, time_pref_prob, weather_pref_prob, age_prob);
                        print!("{}\t{}", in_cpt[ind], in_cpt[32+ind]);
                        println!("\n");
                        ind+=1;
                    }
                }
            }
        }
        Self{cpt: in_cpt}
    }
    
    pub fn h_t(hrs_pref: usize) -> f32{
        let mut t = (hrs_pref as f32)/24.;
        let mut not_t = ((24-hrs_pref) as f32)/24.;
        let mut t_h = t + (not_t*BASE_HUNGER_PROB);
        return (t_h*BASE_HUNGER_PROB)/t;
    }

    pub fn h_not_t(hrs_pref: usize, h_t: f32) -> f32{
        let mut t = (hrs_pref as f32)/24.;
        let mut not_t = ((24-hrs_pref) as f32)/24.;
        let mut h_and_t = h_t * t;
        return (BASE_HUNGER_PROB - h_and_t)/not_t;
    }

    pub fn index_cpt(&self, inc: bool, hook_ct: i32, time_pref: bool, weather_pref: bool, age: f32) -> f32 {
        let mut ind: usize = 0;
        //Looking for prob of increase or not increase?
        if inc == false {
            ind += INC_IND;
        }

        //Looking for prob with diff hook cts
        if hook_ct > 10 {
            ind += HIGH_HOOK;
        }
        else if hook_ct <= 10 && hook_ct > 5 {
            ind += MED_HOOK;
        }
        else if hook_ct <= 5 && hook_ct > 0 {
            ind += LOW_HOOK;
        }

        //Looking for prob with pref/not pref time
        if time_pref == false {
            ind += T_NOT_PREF;
        }

        //Looking for prob with pref/not pref weather
        if weather_pref == false {
            ind += W_NOT_PREF;
        }

        //Looking for prob with old/young fish
        if age >= 50. {
            ind += AGE_OLD;
        }

        //Use index to get correct val to return
        return self.cpt[ind];
    }
}