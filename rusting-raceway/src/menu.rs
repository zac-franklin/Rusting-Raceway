use bevy::{
    app::AppExit,
    ecs::spawn::{SpawnIter, SpawnWith},
    prelude::*,
};

use super::{
    despawn_screen, 
    components::{DisplayQuality, MenuButtonAction, OnDisplaySettingsMenuScreen, 
        OnMainMenuScreen, OnSettingsMenuScreen, OnSoundSettingsMenuScreen, 
        SelectedOption, Volume}, 
    states::{GameState, MenuState, },
    resources::{ExitMenuIcon, FontHandle, RightMenuIcon, WrenchMenuIcon}
};

const NORMAL_BUTTON: Color = Color::srgba(0.15, 0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgba(0.05, 0.05, 0.05, 0.50);
const HOVERED_PRESSED_BUTTON: Color = Color::srgba(0.25, 0.65, 0.25, 0.50);
const PRESSED_BUTTON: Color = Color::srgba(0.35, 0.75, 0.35, 1.0);

const BACKGROUND_COLOR: Color = Color::srgba(0.15, 0.15, 0.15, 0.25);

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const RED_TEXT_COLOR: Color = Color::srgb(0.89, 0.24, 0.12);
const GREEN_TEXT_COLOR: Color = Color::srgb(0.31, 0.68, 0.39);
const BLUE_TEXT_COLOR: Color = Color::srgb(0.41, 0.52, 0.76);

const PARTY_COLORS: &[Color] = &[RED_TEXT_COLOR, GREEN_TEXT_COLOR, BLUE_TEXT_COLOR];

/// Plugin for the menu with "New Game", "Settings", "Quit" sub menus
pub fn menu_plugin(app: &mut App) {
    app
        .init_state::<MenuState>()
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .add_systems(
            Startup,
            load_menu_icons,
        )
        .add_systems(OnEnter(GameState::Menu), 
            (
                menu_setup,
            )
        )
        // Systems to handle the main menu screen
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        // Systems to handle the settings menu screen
        .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
        .add_systems(
            OnExit(MenuState::Settings),
            despawn_screen::<OnSettingsMenuScreen>,
        )
        // Systems to handle the display settings screen
        .add_systems(
            OnEnter(MenuState::SettingsDisplay),
            display_settings_menu_setup,
        )
        .add_systems(
            Update,
            (setting_button::<DisplayQuality>.run_if(in_state(MenuState::SettingsDisplay)),),
        )
        .add_systems(
            OnExit(MenuState::SettingsDisplay),
            despawn_screen::<OnDisplaySettingsMenuScreen>,
        )
        // Systems to handle the sound settings screen
        .add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup)
        .add_systems(
            Update,
            setting_button::<Volume>.run_if(in_state(MenuState::SettingsSound)),
        )
        .add_systems(
            OnExit(MenuState::SettingsSound),
            despawn_screen::<OnSoundSettingsMenuScreen>,
        )
        // Common systems to all screens that handles buttons behavior
        .add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(GameState::Menu)),
        );
}

fn load_menu_icons(mut commands: Commands, asset_server: Res<AssetServer>) {
    let right_image = asset_server.load("textures/right.png");
    commands.insert_resource(RightMenuIcon(right_image));

    let wrench_image = asset_server.load("textures/wrench.png");
    commands.insert_resource(WrenchMenuIcon(wrench_image));

    let exit_image = asset_server.load("textures/exitRight.png");
    commands.insert_resource(ExitMenuIcon(exit_image));
}

/// System to handle changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

/// System to update the settings when a new value for a setting is selected, and mark
/// the button as the one currently selected
fn setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    selected_query: Single<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    let (previous_button, mut previous_button_color) = selected_query.into_inner();
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            *previous_button_color = NORMAL_BUTTON.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}

/// Set initial Menu State
fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

/// split text by char, display char with its own color based on input colors and it's position in string.
fn text_with_party_font(text: &'static str, colors: &'static [Color], mario_font: &Handle<Font>) -> impl Bundle {
    let mario_font_clone = mario_font.clone();

    (
        Node {
            margin: UiRect::all(Val::Px(0.0)),
            ..default()
        },
        Children::spawn(
            SpawnWith( move |parent: &mut ChildSpawner| { 
                for (i, c) in text.chars().enumerate() {
                    let mut tmp = [0; 4];

                    make_stroked_char(parent, c.encode_utf8(&mut tmp), colors[i % colors.len()], &mario_font_clone);
                }
            })
        )
    )
}

/// Add stroke to texts by using a similar concept in CSS( https://css-tricks.com/adding-stroke-to-web-text/#aa-simulation ) with text shadows. 
/// NOTE: Bevy has a text shadow but it doesn't support Vec of shadows like CSS.
fn make_stroked_char(parent: &mut ChildSpawner, text: &str, color: Color, mario_font: &Handle<Font>) {
    parent.spawn(
        Node {
            margin: UiRect::all(Val::Px(0.0)),
            position_type: PositionType::Relative,
            ..default()
        }
    ).with_children( |parent| {
        //Draw bottom right shadow
        parent.spawn(
            (
                (
                Text::new(text),
                TextFont {
                    font: mario_font.clone(),
                    font_size: 67.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 0.0, 0.0)),
                ),
                Node {
                    position_type: PositionType::Absolute,
                    top:Val::Px(-2.0),
                    left:Val::Px(-2.0),
                    ..default()
                },
            )
        );

        //Draw bottom left shadow
        parent.spawn(
            (
                (
                Text::new(text),
                TextFont {
                    font: mario_font.clone(),
                    font_size: 67.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 0.0, 0.0)),
                ),
                Node {
                    position_type: PositionType::Absolute,
                    top:Val::Px(-2.0),
                    left:Val::Px(2.0),
                    ..default()
                },
            )
        );

        //Draw top right shadow
        parent.spawn(
            (
                (
                Text::new(text),
                TextFont {
                    font: mario_font.clone(),
                    font_size: 67.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 0.0, 0.0)),
                ),
                Node {
                    position_type: PositionType::Absolute,
                    top:Val::Px(2.0),
                    left:Val::Px(-2.0),
                    ..default()
                },
            )
        );

        //Draw top left shadow
        parent.spawn(
            (
                (
                Text::new(text),
                TextFont {
                    font: mario_font.clone(),
                    font_size: 67.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 0.0, 0.0)),
                ),
                Node {
                    position_type: PositionType::Absolute,
                    top:Val::Px(2.0),
                    left:Val::Px(2.0),
                    ..default()
                },
            )
        );

        //Draw bottom shadow
        parent.spawn(
            (
                (
                Text::new(text),
                TextFont {
                    font: mario_font.clone(),
                    font_size: 67.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 0.0, 0.0)),
                ),
                Node {
                    position_type: PositionType::Absolute,
                    top:Val::Px(-3.0),
                    left:Val::Px(0.0),
                    ..default()
                },
            )
        );

        //Draw top shadow
        parent.spawn(
            (
                (
                Text::new(text),
                TextFont {
                    font: mario_font.clone(),
                    font_size: 67.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 0.0, 0.0)),
                ),
                Node {
                    position_type: PositionType::Absolute,
                    top:Val::Px(3.0),
                    left:Val::Px(0.0),
                    ..default()
                },
            )
        );

        //Draw right shadow
        parent.spawn(
            (
                (
                Text::new(text),
                TextFont {
                    font: mario_font.clone(),
                    font_size: 67.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 0.0, 0.0)),
                ),
                Node {
                    position_type: PositionType::Absolute,
                    top:Val::Px(0.0),
                    left:Val::Px(-3.0),
                    ..default()
                },
            )
        );

        //Draw left shadow
        parent.spawn(
            (
                (
                Text::new(text),
                TextFont {
                    font: mario_font.clone(),
                    font_size: 67.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 0.0, 0.0)),
                ),
                Node {
                    position_type: PositionType::Absolute,
                    top:Val::Px(0.0),
                    left:Val::Px(3.0),
                    ..default()
                },
            )
        );

        //Draw text shadow
        parent.spawn(
            (
                (
                Text::new(text),
                TextFont {
                    font: mario_font.clone(),
                    font_size: 67.0,
                    ..default()
                },
                TextColor(color),
                ),
                Node {
                    ..default()
                },
            )
        );
    });
}

/// Setup components for initial Menu Screen
fn main_menu_setup(mut commands: Commands, right_icon: Res<RightMenuIcon>, wrench_icon: Res<WrenchMenuIcon>, exit_icon: Res<ExitMenuIcon>, mario_font: Res<FontHandle>) {
    // Common style for all buttons on the screen
    let button_icon_node = Node {
        width: Val::Px(30.0),
        position_type: PositionType::Absolute,
        left: Val::Px(10.0),
        ..default()
    };

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnMainMenuScreen,
        BackgroundColor(BACKGROUND_COLOR),
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            children![
                (
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    children![
                        text_with_party_font("RUSTING", PARTY_COLORS, &mario_font.0),
                        text_with_party_font("RACEWAY", PARTY_COLORS, &mario_font.0),
                    ]
                ),
                (
                    Button,
                    button_node(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Play,
                    children![
                        (ImageNode::new(right_icon.0.clone()), button_icon_node.clone()),
                        (
                            Text::new("New Game"),
                            button_text_style(&mario_font.0),
                        ),
                    ]
                ),
                (
                    Button,
                    button_node(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Settings,
                    children![
                        (ImageNode::new(wrench_icon.0.clone()), button_icon_node.clone()),
                        (
                            Text::new("Settings"),
                            button_text_style(&mario_font.0)
                        ),
                    ]
                ),
                (
                    Button,
                    button_node(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Quit,
                    children![
                        (ImageNode::new(exit_icon.0.clone()), button_icon_node),
                        (Text::new("Quit"), button_text_style(&mario_font.0),),
                    ]
                )
            ]
        )]
    ));
}

/// Setup Components for Settings Menu Screen
fn settings_menu_setup(mut commands: Commands, mario_font: Res<FontHandle>) {
    let mario_font_clone = mario_font.0.clone();

    commands.spawn((
        Node {
            display: Display::Flex,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(BACKGROUND_COLOR),
        OnSettingsMenuScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            Children::spawn((
                Spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect {
                            left: Val::Px(0.),
                            right: Val::Px(0.),
                            top: Val::Px(0.),
                            bottom: Val::Px(50.)
                        },
                        ..default()
                    },
                    children![
                        text_with_party_font("SETTINGS", &[RED_TEXT_COLOR], &mario_font.0)
                    ]
                )),
                // Display three buttons for each action available from the settings menu:
                // - display
                // - sound
                // - back
                SpawnIter(
                [
                    (MenuButtonAction::SettingsDisplay, "Display"),
                    (MenuButtonAction::SettingsSound, "Sound"),
                    (MenuButtonAction::BackToMainMenu, "Back"),
                ]
                .into_iter()
                .map(move |(action, text)| {
                    (
                        Button,
                        button_node(),
                        BackgroundColor(NORMAL_BUTTON),
                        action,
                        children![(Text::new(text), button_text_style(&mario_font_clone.clone()))],
                    )
                })
            )))
        )],
    ));
}

/// Setup Components for Display Settings Screen
fn display_settings_menu_setup(mut commands: Commands, display_quality: Res<DisplayQuality>, mario_font: Res<FontHandle>) {
    let display_quality = *display_quality;
    let mario_font_clone = mario_font.0.clone();
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(BACKGROUND_COLOR),
        OnDisplaySettingsMenuScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            children![
                (
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect {
                            left: Val::Px(0.),
                            right: Val::Px(0.),
                            top: Val::Px(0.),
                            bottom: Val::Px(50.)
                        },
                        ..default()
                    },
                    children![
                        text_with_party_font("DISPLAY", &[GREEN_TEXT_COLOR], &mario_font.0)
                    ]
                ),
                (
                    Node {
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(BACKGROUND_COLOR),
                    Children::spawn((
                        // Display a label for the current setting
                        Spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                padding: UiRect {
                                    left: Val::Px(10.),
                                    right: Val::Px(10.),
                                    top: Val::Px(0.),
                                    bottom: Val::Px(0.)
                                },
                                ..default()
                            },
                            BackgroundColor(BACKGROUND_COLOR),
                            children![(Text::new("Display Quality"), button_text_style(&mario_font.0))]
                        )),
                        SpawnWith(move |parent: &mut ChildSpawner| {
                            for quality_setting in [
                                DisplayQuality::Low,
                                DisplayQuality::Medium,
                                DisplayQuality::High,
                            ] {
                                let mut entity = parent.spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(150.0),
                                        height: Val::Px(65.0),
                                        ..button_node()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    quality_setting,
                                    children![(
                                        Text::new(format!("{quality_setting:?}")),
                                        button_text_style(&mario_font_clone),
                                    )],
                                ));
                                if display_quality == quality_setting {
                                    entity.insert(SelectedOption);
                                }
                            }
                        })
                    ))
                ),
                // Display the back button to return to the settings screen
                (
                    Button,
                    button_node(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::BackToSettings,
                    children![(Text::new("Back"), button_text_style(&mario_font.0))]
                )
            ]
        )],
    ));
}

/// Setup Components for Sound Settings Screen
fn sound_settings_menu_setup(mut commands: Commands, volume: Res<Volume>, mario_font: Res<FontHandle>) {
    let volume = *volume;
    commands.spawn((
        Node {
            display: Display::Flex,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(BACKGROUND_COLOR),
        OnSoundSettingsMenuScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            children![
                (
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect {
                            left: Val::Px(0.),
                            right: Val::Px(0.),
                            top: Val::Px(0.),
                            bottom: Val::Px(50.)
                        },
                        ..default()
                    },
                    children![
                        text_with_party_font("SOUND", &[BLUE_TEXT_COLOR], &mario_font.0),
                    ]
                ),
                (
                    Node {
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(BACKGROUND_COLOR),
                    Children::spawn((
                        Spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                padding: UiRect {
                                    left: Val::Px(10.),
                                    right: Val::Px(10.),
                                    top: Val::Px(0.),
                                    bottom: Val::Px(0.)
                                },
                                ..default()
                            },
                            BackgroundColor(BACKGROUND_COLOR),
                            children![(Text::new("Volume"), button_text_style(&mario_font.0))]
                        )),
                        SpawnWith(move |parent: &mut ChildSpawner| {
                            for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                                let mut entity = parent.spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(30.0),
                                        height: Val::Px(65.0),
                                        ..button_node()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    Volume(volume_setting),
                                ));
                                if volume == Volume(volume_setting) {
                                    entity.insert(SelectedOption);
                                }
                            }
                        })
                    ))
                ),
                (
                    Button,
                    button_node(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::BackToSettings,
                    children![(Text::new("Back"), button_text_style(&mario_font.0))]
                )
            ]
        )],
    ));
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.write(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::Matchmaking);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                MenuButtonAction::SettingsDisplay => {
                    menu_state.set(MenuState::SettingsDisplay);
                }
                MenuButtonAction::SettingsSound => {
                    menu_state.set(MenuState::SettingsSound);
                }
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                MenuButtonAction::BackToSettings => {
                    menu_state.set(MenuState::Settings);
                }
            }
        }
    }
}

fn button_node() -> Node {
    Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

fn button_text_style(mario_font: &Handle<Font>) -> impl Bundle + use<> {
    (
        TextFont {
            font: mario_font.clone(),
            font_size: 33.0,
            ..default()
        },
        TextColor(TEXT_COLOR),
    )
}

