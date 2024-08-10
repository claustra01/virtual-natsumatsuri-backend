use crate::model::{schema, shooting};

fn calc_target(angle: schema::Angle) -> shooting::Target {
  // 偏角から照準座標を計算する
  // 画面中央を原点としてそこから横方向端までの距離を1としている
  // フロント側でこの値にdisplaySizeを掛けて画面サイズに合わせる
  let x = 4.0 * f64::tan(angle.x);
  let y = 4.0 * f64::tan(angle.y);
  shooting::Target { x, y }
}

fn calc_vector(angle: schema::Angle) -> shooting::Vector {
  // 偏角からベクトルを計算する
  let x = f64::sin(angle.x);
  let y = f64::sin(angle.y);
  let z = 0.0; // temporary
  shooting::Vector { x, y, z }
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
