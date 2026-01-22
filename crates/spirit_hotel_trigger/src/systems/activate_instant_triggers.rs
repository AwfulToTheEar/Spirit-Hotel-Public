use bevy::prelude::*;
use crate::{
    components::prelude::IsTrigger,
    events::prelude::{TriggerActivated, TriggerEntered},
};

pub fn activate_instant_triggers(
    e_trigger_entered: On<TriggerEntered>,
    mut commands: Commands,
    q_triggers: Query<&IsTrigger>,
) {

    let Ok(trigger) = q_triggers.get(e_trigger_entered.trigger) else {
        return;
    };

    if matches!(trigger, IsTrigger::Interact) {
        return;
    }

    commands.trigger(TriggerActivated {
        entity: e_trigger_entered.entity,
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