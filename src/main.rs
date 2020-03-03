trait EmailSender {
    fn send_email(&self, msg: &Email) -> Option<String>;
}

#[derive(Debug, Clone)]
struct Email {
    from: String,
    to: String,
    msg: String,
}

#[derive(Debug)]
struct Customer {
    address: String,
    wants_news: bool,
}

fn publish_news(msg: &str, sender: impl EmailSender, customers: &[Customer]) -> Option<i32> {
    let mut count = 0;
    let mut mail = Email {
        from: "Rust Newsletter".to_string(),
        to: "".to_string(),
        msg: msg.to_string(),
    };
    for customer in customers {
        if !customer.wants_news {
            continue;
        }
        mail.to = customer.address.to_string();
        if sender.send_email(&mail).is_none() {
            return None;
        }
        count += 1;
    }
    Some(count)
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockEmailSender {
        send_emails: RefCell<Vec<Email>>,
    }
    impl MockEmailSender {
        fn new() -> Self {
            MockEmailSender {
                send_emails: RefCell::new(Vec::new()),
            }
        }
    }

    impl EmailSender for MockEmailSender {
        fn send_email(&self, msg: &Email) -> Option<String> {
            self.send_emails.borrow_mut().push(msg.clone());
            Some("200 OK".to_string())
        }
    }


    #[test]
    fn sends_zero_to_zero_customers() {
        let sent = publish_news("hello world!", &MockEmailSender::new(), &[]);
        assert_eq!(Some(0), sent);
    }
}
