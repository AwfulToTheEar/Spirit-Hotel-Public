use bevy::prelude::*;
use crate::{
    components::prelude::IsTrigger,
    events::prelude::{TriggerActivated, TriggerEntered},
};
use crate::components::prelude::{InteractTrigger, ThrottleTrigger, TriggerFor, TriggerHasBeenActivated};

pub fn activate_instant_triggers(
    e_trigger_entered: On<TriggerEntered>,
    mut commands: Commands,
    mut q_triggers: Query<(Option<&TriggerFor>, Option<&mut ThrottleTrigger>), (Without<InteractTrigger>, With<IsTrigger>)>,
) {

    let Ok((option_trigger_for, option_throttle_trigger)) = q_triggers.get_mut(e_trigger_entered.trigger) else {
        return;
    };

    if let Some(trigger_for) = option_trigger_for && trigger_for.0 != e_trigger_entered.entity {
        return;
    }

    if let Some(mut throttle_trigger) = option_throttle_trigger {
        if throttle_trigger.timer.is_some() {
            return;
        }

        throttle_trigger.timer = Some(Timer::from_seconds(throttle_trigger.throttle, TimerMode::Once));
    }

    commands.trigger(TriggerActivated {
        entity: e_trigger_entered.entity,
        trigger: e_trigger_entered.trigger,
    });

    commands.entity(e_trigger_entered.trigger).insert_if_new(TriggerHasBeenActivated);
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