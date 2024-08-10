use crate::model::{schema, shooting};

fn calc_target(angle: schema::Angle) -> schema::Target {
  // 偏角から照準座標を計算する
  // 画面中央を原点としてそこから横方向端までの距離を1としている
  // フロント側でこの値にdisplaySizeを掛けて画面サイズに合わせる
  let x = 4 * f64::tan(angle.x);
  let y = 4 * f64::tan(angle.y);
  target { x, y }
}

fn calc_vector(angle: schema::Angle) -> schema::Vector {
  // 偏角からベクトルを計算する
  let x = f64::sin(angle.x);
  let y = f64::sin(angle.y);
  let z = 0; // temporary
  vector { x, y, z }
}

pub fn build_pointer_schema(msg: schema::Schema) -> shooting::PointerSchema {
  pointer_schema {
    id: msg.id,
    message_type: MessageType::Pointer,
    target: calc_target(msg.angle),
  }
}

pub fn build_action_schema(msg: schema::Schema) -> shooting::ActionSchema {
  action_schema {
    id: msg.id,
    message_type: MessageType::Action,
    target: calc_target(msg.angle),
    vector: calc_vector(msg.angle),
  }
}
