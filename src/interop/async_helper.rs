use std::time::Duration;

use cxx_qt_lib::QString;
use tokio::{
    sync::watch::{channel, Receiver, Sender},
    task::JoinHandle,
    time::sleep,
};

use crate::languatool::{
    client::LanguageToolClient,
    service::{Message, Suggestion},
};

pub struct AsyncHelperRust {
    pub message_sender: Sender<Message>,
    pub message_receiver: Receiver<Message>,
    pub suggestion_sender: Sender<Suggestion>,
    pub suggestion_receiver: Receiver<Suggestion>,

    pub handle: Option<JoinHandle<()>>,
}

impl Default for AsyncHelperRust {
    fn default() -> Self {
        let (message_sender, message_receiver) = channel::<Message>(Message::default());
        let (suggestion_sender, suggestion_receiver) = channel::<Suggestion>(Suggestion::default());

        Self {
            message_sender,
            message_receiver,
            suggestion_sender,
            suggestion_receiver,
            handle: None,
        }
    }
}

impl Drop for AsyncHelperRust {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.as_ref() {
            handle.abort();
        }
    }
}

impl AsyncHelperRust {
    pub fn start_async_worker(&mut self) {
        let mut message_receiver = self.message_receiver.clone();
        let suggestion_sender = self.suggestion_sender.clone();
        self.handle = Some(tokio::spawn(async move {
            let mut last_text = QString::default();
            loop {
                let _ = message_receiver.changed().await;

                loop {
                    let debounce = sleep(Duration::from_millis(300));
                    tokio::pin!(debounce);
                    tokio::select! {
                        _ = &mut debounce => break,
                        _ = message_receiver.changed() => {}
                    }
                }

                let message = message_receiver.borrow().clone();
                let Message(text) = message;

                if text == last_text || text.trimmed().is_empty() {
                    continue;
                }

                last_text = text.clone();

                let suggestions = LanguageToolClient::get_recommendation(text.to_string()).await;
                let _ = suggestion_sender.send(Suggestion(suggestions));
            }
        }));
    }
}
