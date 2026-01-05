use bevy::prelude::*;
use crate::{
    components::prelude::IsTrigger,
    messages::prelude::{TriggerActivatedMessage, TriggerEnteredMessage},
};

pub fn activate_instant_triggers(
    mut e_trigger_entered: MessageReader<TriggerEnteredMessage>,
    mut e_trigger_activated: MessageWriter<TriggerActivatedMessage>,
    q_triggers: Query<&IsTrigger>,
) {
    for e in e_trigger_entered.read() {
        let Ok(trigger) = q_triggers.get(e.trigger) else {
            continue;
        };

        if matches!(trigger, IsTrigger::Interact) {
            return;
        }

        e_trigger_activated.write(TriggerActivatedMessage {
            entity: e.entity,
            trigger: e.trigger,
        });
    }
}

#[test]
fn interact_trigger_did_not_activate_trigger() {
    let mut app = App::new();

    app.add_message::<TriggerEnteredMessage>()
        .add_message::<TriggerActivatedMessage>()

        .add_systems(Update, activate_instant_triggers);

    let trigger_entity = app.world_mut()
        .spawn(IsTrigger::Interact)
        .id();

    app.world_mut().resource_mut::<Messages<TriggerEnteredMessage>>().write(TriggerEnteredMessage {
        trigger: trigger_entity,
        entity: trigger_entity
    });

    app.update();

    let trigger_activated_messages = app.world().resource::<Messages<TriggerActivatedMessage>>();
    let mut trigger_activated_cursor = trigger_activated_messages.get_cursor();
    let trigger_activated = trigger_activated_cursor.read(trigger_activated_messages).next();

    assert!(trigger_activated.is_none());
}

#[test]
fn instant_trigger_activated_trigger() {
    let mut app = App::new();

    app.add_message::<TriggerEnteredMessage>()
        .add_message::<TriggerActivatedMessage>()

        .add_systems(Update, activate_instant_triggers);

    let trigger_entity = app.world_mut()
        .spawn(IsTrigger::Instant)
        .id();

    app.world_mut().resource_mut::<Messages<TriggerEnteredMessage>>().write(TriggerEnteredMessage {
        trigger: trigger_entity,
        entity: trigger_entity
    });

    app.update();

    let trigger_activated_messages = app.world().resource::<Messages<TriggerActivatedMessage>>();
    let mut trigger_activated_cursor = trigger_activated_messages.get_cursor();
    let trigger_activated = trigger_activated_cursor.read(trigger_activated_messages).next().unwrap();

    assert_eq!(trigger_activated.trigger, trigger_entity);
}
