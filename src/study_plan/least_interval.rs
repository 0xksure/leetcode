//! Not finished

pub struct Solution;

impl Solution {
    pub fn least_interval(mut tasks: Vec<char>, n: i32) -> i32 {
        // given tasks
        // ["A","A","A","B","C","D","A"]
        // idle n between equal tasks
        // Intuition
        // start with first task, then find next different task
        // pop tasks when done
        if n == 0 {
            return tasks.len() as i32;
        }
        let mut task_list = Vec::new();
        let mut task = tasks[0];
        let mut time = 0;
        tasks.remove(0);

        while tasks.len() != 0 {
            // find next different task
            for i in 0..tasks.len() {
                if tasks[i] != task {
                    task = tasks[i];
                    tasks.remove(i);
                    task_list.push(task);
                    time += 1;
                    break;
                }
            }
            time += n;
            task_list.push(tasks[(tasks.len() - 1)]);
            tasks.pop();
        }

        println!("{:?}", task_list);
        time
    }
}
