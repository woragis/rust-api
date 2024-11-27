use faker_rand::en_us::names::FullName;

async fn fake_names() {
    let random_name = rand::random::<FullName>();
    println!("Full name: {}", random_name);
}

fn gen_fake_user() {}
