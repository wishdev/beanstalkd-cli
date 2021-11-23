use beanstalkd::Beanstalkd;

pub fn peek(beanstalkd: &mut Beanstalkd, type_or_id: String, tube: &str) {
    if tube != "default" {
        let _ = beanstalkd.tube(tube);
    }

    let (_id, message) = beanstalkd.peek(&type_or_id).unwrap();
    println!("{}", message);
}
