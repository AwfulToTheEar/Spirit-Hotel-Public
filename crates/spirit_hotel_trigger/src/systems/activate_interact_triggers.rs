use bevy::prelude::*;
use crate::{
    components::prelude::{InsideTriggers, IsTrigger},
    messages::prelude::TriggerActivatedMessage,
};

pub fn activate_interact_triggers(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut e_trigger_activated: MessageWriter<TriggerActivatedMessage>,
    q_inside_triggers: Query<(Entity, &InsideTriggers)>,
    q_triggers: Query<&IsTrigger>,
) {
    if !keyboard_input.just_pressed(KeyCode::KeyE) {
        return;
    }

    for (entity, inside_triggers) in q_inside_triggers.iter() {
        for trigger_entity in inside_triggers.entities.iter() {
            let Ok(trigger) = q_triggers.get(*trigger_entity) else {
                continue;
            };

            if matches!(trigger, IsTrigger::Instant) {
                continue;
            }

            e_trigger_activated.write(TriggerActivatedMessage {
                entity,
                trigger: *trigger_entity,
            });
        }
    }
}

#[test]
fn interact_trigger_activated_trigger() {
    let mut app = App::new();

    app.insert_resource(ButtonInput::<KeyCode>::default())

        .add_message::<TriggerActivatedMessage>()

        .add_systems(Update, activate_interact_triggers);

    let trigger_entity = app.world_mut().spawn(IsTrigger::Interact).id();
    let inside_triggers_entity = app.world_mut().spawn(InsideTriggers { entities: vec![trigger_entity] }).id();

    app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::KeyE);

    app.update();

    let trigger_activated_messages = app.world().resource::<Messages<TriggerActivatedMessage>>();
    let mut trigger_activated_cursor = trigger_activated_messages.get_cursor();
    let trigger_activated = trigger_activated_cursor.read(trigger_activated_messages).next().unwrap();

    assert_eq!(trigger_activated.trigger, trigger_entity);
    assert_eq!(trigger_activated.entity, inside_triggers_entity);
}

#[test]
fn instant_trigger_did_not_activate_trigger() {
    let mut app = App::new();

    app.insert_resource(ButtonInput::<KeyCode>::default())

        .add_message::<TriggerActivatedMessage>()

        .add_systems(Update, activate_interact_triggers);

    let trigger_entity = app.world_mut().spawn(IsTrigger::Instant).id();
    app.world_mut().spawn(InsideTriggers { entities: vec![trigger_entity] });

    app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::KeyE);

    app.update();

    let trigger_activated_messages = app.world().resource::<Messages<TriggerActivatedMessage>>();
    let mut trigger_activated_cursor = trigger_activated_messages.get_cursor();
    let trigger_activated = trigger_activated_cursor.read(trigger_activated_messages).next();

    assert!(trigger_activated.is_none());
}