use crate::model::{schema, shooting};
use std::f64::consts::PI;

fn calc_target(angle: schema::Angle) -> shooting::Target {
    // 偏角から照準座標を計算する
    // 画面中央を原点としてそこから横方向端までの距離を1としている
    // フロント側でこの値にdisplaySizeを掛けて画面サイズに合わせる
    let x = 4.0 * f64::tan(PI * angle.x / 180.0);
    let y = 4.0 * f64::tan(PI * angle.y / 180.0);
    shooting::Target { x, y }
}

fn calc_vector(angle: schema::Angle) -> shooting::Vector {
    // TODO: temporary
    shooting::Vector {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    }
}

pub fn build_pointer_schema(msg: schema::Schema) -> shooting::PointerSchema {
    shooting::PointerSchema {
        id: msg.id,
        message_type: shooting::MessageType::Pointer,
        target: calc_target(msg.angle.clone()),
    }
}

pub fn build_action_schema(msg: schema::Schema) -> shooting::ActionSchema {
    shooting::ActionSchema {
        id: msg.id,
        message_type: shooting::MessageType::Action,
        target: calc_target(msg.angle.clone()),
        vector: calc_vector(msg.angle.clone()),
    }
}
