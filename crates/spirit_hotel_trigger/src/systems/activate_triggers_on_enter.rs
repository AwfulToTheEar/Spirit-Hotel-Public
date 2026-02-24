use crate::components::prelude::{
    InteractTrigger, SubTriggerOf, SubTriggers, ThrottleTrigger, ThrottleTriggerTimer, TriggerFor,
    TriggerHasBeenActivated, TriggerHeldFor, WaitForExternalAction,
};
use crate::{
    components::prelude::IsTrigger,
    events::prelude::{TriggerActivated, TriggerEntered},
};
use bevy::prelude::*;

pub fn activate_instant_triggers(
    e_trigger_entered: On<TriggerEntered>,
    mut commands: Commands,
    mut q_triggers: Query<
        (
            Option<&TriggerFor>,
            Option<&SubTriggers>,
            Option<&ThrottleTrigger>,
            Has<ThrottleTriggerTimer>,
        ),
        (
            Without<InteractTrigger>,
            Without<WaitForExternalAction>,
            With<IsTrigger>,
        ),
    >,
    q_sub_triggers: Query<(
        &SubTriggerOf,
        Has<TriggerHasBeenActivated>,
        Has<TriggerHeldFor>,
    )>,
) {
    let Ok((
        option_trigger_for,
        option_sub_triggers,
        option_throttle_trigger,
        has_throttle_trigger_timer,
    )) = q_triggers.get_mut(e_trigger_entered.trigger)
    else {
        return;
    };

    if let Some(sub_triggers) = option_sub_triggers {
        for sub_trigger in sub_triggers.iter() {
            let Ok((sub_trigger_of, has_trigger_been_activated, has_trigger_held_for)) =
                q_sub_triggers.get(sub_trigger)
            else {
                continue;
            };

            if !has_trigger_been_activated {
                return;
            }

            if sub_trigger_of.should_be_held && !has_trigger_held_for {
                return;
            }
        }
    }

    if let Some(trigger_for) = option_trigger_for
        && trigger_for.0 != e_trigger_entered.by
    {
        return;
    }

    if has_throttle_trigger_timer {
        return;
    }

    if let Some(throttle_trigger) = option_throttle_trigger {
        commands
            .entity(e_trigger_entered.trigger)
            .insert(ThrottleTriggerTimer(Timer::from_seconds(
                throttle_trigger.0,
                TimerMode::Once,
            )));
    }

    commands.trigger(TriggerActivated {
        by: Some(e_trigger_entered.by),
        trigger: e_trigger_entered.trigger,
    });
}

/*
#[test]
fn interact_trigger_did_not_activate_trigger() {
    let mut app = App::new();

    app.add_message::<TriggerEntered>()
        .add_message::<TriggerActivated>()

        .add_systems(Update, activate_instant_triggers);

    let trigger_entity = app.world_mut()
        .spawn(IsTrigger::Interact)
        .id();

    app.world_mut().resource_mut::<Messages<TriggerEntered>>().write(TriggerEntered {
        trigger: trigger_entity,
        entity: trigger_entity
    });

    app.update();

    let trigger_activated_messages = app.world().resource::<Messages<TriggerActivated>>();
    let mut trigger_activated_cursor = trigger_activated_messages.get_cursor();
    let trigger_activated = trigger_activated_cursor.read(trigger_activated_messages).next();

    assert!(trigger_activated.is_none());
}

#[test]
fn instant_trigger_activated_trigger() {
    let mut app = App::new();

    app.add_message::<TriggerEntered>()
        .add_message::<TriggerActivated>()

        .add_systems(Update, activate_instant_triggers);

    let trigger_entity = app.world_mut()
        .spawn(IsTrigger::Instant)
        .id();

    app.world_mut().resource_mut::<Messages<TriggerEntered>>().write(TriggerEntered {
        trigger: trigger_entity,
        entity: trigger_entity
    });

    app.update();

    let trigger_activated_messages = app.world().resource::<Messages<TriggerActivated>>();
    let mut trigger_activated_cursor = trigger_activated_messages.get_cursor();
    let trigger_activated = trigger_activated_cursor.read(trigger_activated_messages).next().unwrap();

    assert_eq!(trigger_activated.trigger, trigger_entity);
}
*/
