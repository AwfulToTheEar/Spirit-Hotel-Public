use std::ops::Add;
use std::time::Duration;
use bevy::prelude::*;
use crate::{
    components::prelude::{InsideTriggers, IsTrigger},
    events::prelude::TriggerActivated,
};
use crate::components::prelude::{ContinuousTrigger, InteractTrigger, ThrottleTrigger, TriggerFor, TriggerHasBeenActivated};

pub fn activate_interact_triggers(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    q_inside_triggers: Query<(Entity, &InsideTriggers)>,
    mut q_triggers: Query<(Option<&TriggerFor>, Option<&mut ThrottleTrigger>, Option<&mut ContinuousTrigger>, Has<InteractTrigger>), With<IsTrigger>>,
) {
    for (entity, inside_triggers) in q_inside_triggers.iter() {
        for trigger_entity in inside_triggers.entities.iter() {
            let Ok((option_trigger_for, option_throttle_trigger, option_continuous_trigger, has_interact_trigger)) = q_triggers.get_mut(*trigger_entity) else {
                continue;
            };

            if has_interact_trigger && !keyboard_input.just_pressed(KeyCode::KeyE) || !has_interact_trigger && option_continuous_trigger.is_none() {
                continue;
            }

            if let Some(mut throttle_trigger) = option_throttle_trigger {
                if throttle_trigger.timer.is_some() {
                    continue;
                }

                throttle_trigger.timer = Some(Timer::from_seconds(throttle_trigger.throttle, TimerMode::Once));
            }

            if let Some(mut continuous_trigger) = option_continuous_trigger {
                let duration = continuous_trigger.0.get_or_insert(Duration::ZERO);

                *duration = duration.add(time.delta());
            }

            if let Some(trigger_for) = option_trigger_for && trigger_for.0 != entity {
                continue;
            }

            commands.trigger(TriggerActivated {
                entity,
                trigger: *trigger_entity,
            });

            commands.entity(*trigger_entity).insert_if_new(TriggerHasBeenActivated);
        }
    }
}

/*
#[test]
fn interact_trigger_activated_trigger() {
    let mut app = App::new();

    app.insert_resource(ButtonInput::<KeyCode>::default())

        .add_message::<TriggerActivated>()

        .add_systems(Update, activate_interact_triggers);

    let trigger_entity = app.world_mut().spawn(IsTrigger::Interact).id();
    let inside_triggers_entity = app.world_mut().spawn(InsideTriggers { entities: vec![trigger_entity] }).id();

    app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::KeyE);

    app.update();

    let trigger_activated_messages = app.world().resource::<Messages<TriggerActivated>>();
    let mut trigger_activated_cursor = trigger_activated_messages.get_cursor();
    let trigger_activated = trigger_activated_cursor.read(trigger_activated_messages).next().unwrap();

    assert_eq!(trigger_activated.trigger, trigger_entity);
    assert_eq!(trigger_activated.entity, inside_triggers_entity);
}

#[test]
fn instant_trigger_did_not_activate_trigger() {
    let mut app = App::new();

    app.insert_resource(ButtonInput::<KeyCode>::default())

        .add_message::<TriggerActivated>()

        .add_systems(Update, activate_interact_triggers);

    let trigger_entity = app.world_mut().spawn(IsTrigger::Instant).id();
    app.world_mut().spawn(InsideTriggers { entities: vec![trigger_entity] });

    app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::KeyE);

    app.update();

    let trigger_activated_messages = app.world().resource::<Messages<TriggerActivated>>();
    let mut trigger_activated_cursor = trigger_activated_messages.get_cursor();
    let trigger_activated = trigger_activated_cursor.read(trigger_activated_messages).next();

    assert!(trigger_activated.is_none());
}
*/