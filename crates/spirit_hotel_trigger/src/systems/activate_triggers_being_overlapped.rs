use crate::components::prelude::{ContinuousTrigger, InteractTrigger, SubTriggerOf, SubTriggers, ThrottleTrigger, ThrottleTriggerTimer, TriggerFor, TriggerHasBeenActivated, TriggerHeldFor, WaitForExternalAction};
use crate::{
    components::prelude::{InsideTriggers, IsTrigger},
    events::prelude::TriggerActivated,
};
use bevy::ecs::relationship::RelationshipSourceCollection;
use bevy::prelude::*;
use std::time::Duration;

pub fn activate_interact_triggers(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    q_inside_triggers: Query<(Entity, &InsideTriggers)>,
    mut q_triggers: Query<
        (
            Option<&TriggerFor>,
            Option<&SubTriggers>,
            Option<&ThrottleTrigger>,
            Has<ThrottleTriggerTimer>,
            Has<ContinuousTrigger>,
            Has<TriggerHeldFor>,
            Has<InteractTrigger>,
        ),
        (With<IsTrigger>, Without<WaitForExternalAction>)
    >,
    q_sub_triggers: Query<(
        &SubTriggerOf,
        Has<TriggerHasBeenActivated>,
        Has<TriggerHeldFor>,
    )>,
) {
    for (entity, inside_triggers) in q_inside_triggers.iter() {
        for trigger_entity in inside_triggers.entities.iter() {
            let Ok((
                option_trigger_for,
                option_sub_triggers,
                option_throttle_trigger,
                has_throttle_trigger_timer,
                has_continuous_trigger,
                has_trigger_held_for,
                has_interact_trigger,
            )) = q_triggers.get_mut(trigger_entity)
            else {
                continue;
            };

            if let Some(sub_triggers) = option_sub_triggers {
                let mut sub_triggers_not_valid = false;
                for sub_trigger in sub_triggers.iter() {
                    let Ok((sub_trigger_of, has_trigger_been_activated, has_trigger_held_for)) =
                        q_sub_triggers.get(sub_trigger)
                    else {
                        continue;
                    };

                    if !has_trigger_been_activated {
                        sub_triggers_not_valid = true;
                        break;
                    }

                    if sub_trigger_of.should_be_held && !has_trigger_held_for {
                        sub_triggers_not_valid = true;
                        break;
                    }
                }

                if sub_triggers_not_valid {
                    continue;
                }
            }

            if has_interact_trigger
                && !has_continuous_trigger
                && !keyboard_input.just_pressed(KeyCode::KeyE)
                || !has_interact_trigger && !has_continuous_trigger
                || has_interact_trigger
                    && has_continuous_trigger
                    && !keyboard_input.pressed(KeyCode::KeyE)
            {
                continue;
            }

            if has_throttle_trigger_timer {
                continue;
            }

            if let Some(throttle_trigger) = option_throttle_trigger {
                commands
                    .entity(trigger_entity)
                    .insert(ThrottleTriggerTimer(Timer::from_seconds(
                        throttle_trigger.0,
                        TimerMode::Once,
                    )));
            }

            if has_continuous_trigger && !has_trigger_held_for {
                commands
                    .entity(trigger_entity)
                    .insert(TriggerHeldFor(Duration::ZERO));
            }

            if let Some(trigger_for) = option_trigger_for
                && trigger_for.0 != entity
            {
                continue;
            }

            commands.trigger(TriggerActivated {
                by: Some(entity),
                trigger: trigger_entity,
            });
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
