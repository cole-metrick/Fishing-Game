use bevy::prelude::*;
use crate::gameday::*;
use crate::inventory::*;
use crate::weather::*;
use crate::interface::*;

#[derive(Component)]
pub struct MoneyDisplay;

#[derive(Component)]
pub struct ClockDisplay;

#[derive(Component)]
pub struct WeatherDisplay;

#[derive(Component)]
pub struct HintDisplay;

pub fn spawn_money_display(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "Money: 0",
            TextStyle {
                font: asset_server.load("fonts/pixel.ttf"),
                font_size: 65.0,
                color: Color::srgb(0.0, 0.0, 0.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(5.0),
            ..default()
        }),
        MoneyDisplay,
    ));
}

pub fn spawn_clock_display(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "Time: 0",
            TextStyle {
                font: asset_server.load("fonts/pixel.ttf"),
                font_size: 65.0,
                color: Color::srgb(0.0, 0.0, 0.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Px(5.0),
            ..default()
        }),
        ClockDisplay,
    ));
}

pub fn spawn_weather_display(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "Weather: 0",
            TextStyle {
                font: asset_server.load("fonts/pixel.ttf"),
                font_size: 65.0,
                color: Color::srgb(0.0, 0.0, 0.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(5.0),
            ..default()
        }),
        WeatherDisplay,
    ));
}

pub fn spawn_hint(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let mut text =         TextBundle::from_section(
        "Ocean fishing requires the surf rod",
        TextStyle {
            font: asset_server.load("fonts/pixel.ttf"),
            font_size: 40.0,
            color: Color::srgb(0.0, 0.0, 0.0),
        },
    )
    .with_style(Style {
        position_type: PositionType::Absolute,
        bottom: Val::Px(20.0),
        left: Val::Px(30.0),
        ..default()
    });

    text.visibility = Visibility::Hidden;

    commands.spawn((
        text,
        HintDisplay,
    ));
}


pub fn update_money_display(
    player_inventory: Query<&mut PlayerInventory>,
    mut query: Query<&mut Text, With<MoneyDisplay>>,
) {
    let mut text = query.single_mut();
    let inventory_info = player_inventory.single();
    text.sections[0].value = format!("Money: {}", inventory_info.coins);
}

pub fn update_clock_display(
    time: Res<GameDayTimer>,
    mut query: Query<(&mut Text, &mut Visibility), With<ClockDisplay>>,
    interface: Res<State<CurrentInterface>>,
) {
    let (mut text, mut visibility) = query.single_mut();
    text.sections[0].value = format!("Hour: {}", time.hour);
    if interface.eq(&CurrentInterface::Shop) {
        *visibility = Visibility::Hidden;
    }
    else {
        *visibility = Visibility::Visible;
    }
}

pub fn update_weather_display(
    weather: Res<WeatherState>,
    current_region: Res<State<Region>>,
    mut query: Query<(&mut Text, &mut Visibility), With<WeatherDisplay>>,
    interface: Res<State<CurrentInterface>>,
) {
    let (mut text, mut visibility) = query.single_mut();
    if interface.eq(&CurrentInterface::Shop) {
        *visibility = Visibility::Hidden;
    }
    else {
        *visibility = Visibility::Visible;
        let region = current_region.get();
        let current_weather = weather.weather_by_region.get(region).unwrap_or(&Weather::Sunny);
        let region_name = match region {
            Region::West => "West",
            Region::Central => "Central",
            Region::Shore => "Shore",
        };
        let weather_description = match current_weather {
            Weather::Cloudy => "Cloudy",
            Weather::Rainy => "Rainy",
            Weather::Thunderstorm => "Thunderstorm",
            Weather::Sunny=> "Sunny",
        };
        text.sections[0].value = format!("Region: {} | Weather: {}", region_name, weather_description);
    }
}