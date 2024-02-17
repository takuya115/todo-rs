pub mod create_todo;

use crate::gateway::Gateway;

/// インタラクター
/// 以下を提供する
/// - ハンドラから各ユースケースへのアクセス
/// - 各ユースケースからビジネス・ドメインロジック(Gateway)へのアクセス
pub struct Interactor {
    pub gateway: Box<dyn Gateway>,
}
