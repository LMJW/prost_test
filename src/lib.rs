pub(crate) mod protogen;

use protogen::player;

pub fn create_large_shirt(_: String) -> player::PlayerInGameAction {
    let shirt = player::PlayerInGameAction::default();
    shirt
}

#[cfg(test)]
mod test{

    #[test]
    fn test_protobuf_serialize(){
        use super::*;
        use prost::Message;
        use bytes;
        let action = player::PlayerInGameAction{
            target_player_id: 10,
            action_type: "test".to_owned(),
            message: "testing message".to_owned(),
        };
        let v = action.encode_to_vec();
        println!("encoded message: {:?}",v);

        let de = player::PlayerInGameAction::decode(bytes::Bytes::from(v));
        println!("decoded protobuf message: {:?}", de);
    }
}