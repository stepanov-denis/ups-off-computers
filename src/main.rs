use online::sync::check;
use postgres::{Client, Error, NoTls};

/// Records the event "Происходит выключение компьютеров!" in the sql table "события_авр".
fn event_ups_off_computers() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
    let event = "Происходит выключение компьютеров!";
    client.execute("INSERT INTO события_авр (событие) VALUES ($1)", &[&event])?;

    for row in client.query(
        "SELECT событие, время_и_дата FROM события_авр ORDER BY время_и_дата DESC limit 1",
        &[],
    )? {
        let event: &str = row.get(0);

        println!("Запись в табл. события_авр: {}", event);
    }
    Ok(())
}

/// Records log "Происходит выключение компьютеров!" in the sql table "журнал_работы_приложения".
fn log_ups_off_computers() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
    let event = "Происходит выключение компьютеров!";
    client.execute(
        "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
        &[&event],
    )?;

    for row in client
        .query(
            "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
            &[],
        )
        ?
    {
        let event: &str = row.get(0);

        println!("Запись в табл. журнал_работы_приложения: {}", event);
    }
    Ok(())
}

/// Records log "Отправлено SMS сообщение: /Происходит выключение компьютеров!/ на номер +79139402913" in the sql table "журнал_работы_приложения".
fn log_send_sms_ups_off_computers() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
    let event =
        "Отправлено SMS сообщение: /Происходит выключение компьютеров!/ на номер +79139402913";
    client.execute(
        "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
        &[&event],
    )?;

    for row in client
        .query(
            "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
            &[],
        )
        ?
    {
        let event: &str = row.get(0);

        println!("Запись в табл. журнал_работы_приложения: {}", event);
    }
    Ok(())
}

/// Records log "Server error! Ошибка! SMS уведомление не было отправлено!" in the sql table "журнал_работы_приложения".
fn log_server_err() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
    let event = "Server error! Ошибка! SMS уведомление не было отправлено!";
    client.execute(
        "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
        &[&event],
    )?;

    for row in client
        .query(
            "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
            &[],
        )
        ?
    {
        let event: &str = row.get(0);

        println!("Запись в табл. журнал_работы_приложения: {}", event);
    }
    Ok(())
}

/// Records log "Http request status error! Ошибка! SMS уведомление не было отправлено!" in the sql table "журнал_работы_приложения".
pub fn log_request_status_err() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
    let event = "Http request status error! Ошибка! SMS уведомление не было отправлено!";
    client.execute(
        "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
        &[&event],
    )?;

    for row in client
        .query(
            "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
            &[],
        )
        ?
    {
        let event: &str = row.get(0);

        println!("Запись в табл. журнал_работы_приложения: {}", event);
    }
    Ok(())
}

/// Records log "Ошибка! Доступ к интернету отсутствует! Http запрос не был выполнен! SMS уведомление не было отправлено!" in the sql table "журнал_работы_приложения".
pub fn log_internet_err() -> Result<(), Error> {
    let mut client =
        Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls).unwrap();
    let event = "Ошибка! Доступ к интернету отсутствует! Http запрос не был выполнен! SMS уведомление не было отправлено!";
    client
        .execute(
            "INSERT INTO журнал_работы_приложения (событие) VALUES ($1)",
            &[&event],
        )
        .unwrap();

    for row in client
        .query(
            "SELECT событие, время_и_дата FROM журнал_работы_приложения ORDER BY время_и_дата DESC limit 1",
            &[],
        )
        .unwrap()
    {
        let event: &str = row.get(0);

        println!("Запись в табл. журнал_работы_приложения: {}", event);
    }
    Ok(())
}

/// Executes an http get request to the ClickSend SMS mailing service provider to send the message "Computers are shutting down!".
fn main() -> Result<(), Error> {
    println!("Происходит выключение компьютеров!");
    event_ups_off_computers();
    log_ups_off_computers();
    if check(None).is_ok() {
        let resp = reqwest::blocking::get("https://api-mapper.clicksend.com/http/v2/send.php?method=http&username=development-service@yandex.ru&key=1E82A334-89D8-985C-526B-712DB70A713D&to=+79139402913&message=Происходит+выключение+компьютеров!").unwrap();
        if resp.status().is_success() {
            println!("Http запрос выполнен успешно");
            println!(
                "Отправлено SMS сообщение: /Происходит выключение компьютеров!/ на номер +79139402913"
            );
            log_send_sms_ups_off_computers();
        } else if resp.status().is_server_error() {
            println!("Server error!");
            println!("Ошибка! SMS уведомление не было отправлено!");
            log_server_err();
        } else {
            println!("Status http request: {}", resp.status());
            println!("Ошибка! SMS уведомление не было отправлено!");
            log_request_status_err();
        }
    } else {
        println!("Ошибка! Доступ к интернету отсутствует!");
        println!("Ошибка! Http запрос не был выполнен!");
        println!("Ошибка! SMS уведомление не было отправлено!");
        log_internet_err();
    }
    Ok(())
}
