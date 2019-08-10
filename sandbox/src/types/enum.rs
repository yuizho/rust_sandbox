fn main() {
    #[derive(Debug, PartialEq)]
    enum Weekday {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
    }

    fn say_samething(weekday: Weekday) {
        if weekday == Weekday::Friday {
            println!("TGIF!");
        } else {
            println!("まだ{:?}か", weekday);
        }
    }
    say_samething(Weekday::Friday);

    type UserName = String;
    #[derive(Debug)]
    enum Task {
        Open,
        AssignedTo(UserName),
        Working {
            assignee: UserName,
            remaining_hours: u16,
        },
        Done,
    }

    let tasks = vec![
        Task::AssignedTo(String::from("junko")),
        Task::Working {
            assignee: String::from("hiro"),
            remaining_hours: 18,
        },
        Task::Done,
    ];

    for (i, task) in tasks.iter().enumerate() {
        match task {
            Task::AssignedTo(assignee) => println!(
                "タスク{}は{}さんにアサインサれています",
                i, assignee
            ),
            Task::Working {
                assignee,
                remaining_hours,
            } => println!(
                "タスク{}は{}さんが作業中です。残り{}時間の見込み",
                i, assignee, remaining_hours
            ),
            _ => println!(
                "タスク{}はその他のステータス( {:?} )です",
                i, task
            ),
        }
    }
}
