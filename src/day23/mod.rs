use super::common::Step;
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("12521"),
        Step::Second => String::from("44169"),
    }
}

const DISP: [char; 5] = ['.', 'A', 'B', 'C', 'D'];
const MULT: [u32; 4] = [1, 10, 100, 1000];

#[derive(Clone)]
struct State {
    rooms: Vec<Vec<usize>>,
    hall: Vec<usize>,
    ready_in: HashMap<usize, usize>,
    ready_out: HashMap<usize, usize>,
    full_rooms: HashSet<usize>,
}

impl State {
    fn new(rooms: &[Vec<usize>], hall: &[usize]) -> State {
        let rooms = rooms.to_owned();
        let hall = hall.to_owned();
        // Rooms which are ready to accept occupants
        let mut ready_in = HashMap::new();
        // Rooms which have somebody ready to move to the hall
        let mut ready_out = HashMap::new();
        // Rooms which are full of the right occupants
        let mut full_rooms = HashSet::new();
        for (r, room) in rooms.iter().enumerate() {
            let mut included = false;
            for i in (0..room.len()).rev() {
                if room[i] == 0 {
                    ready_in.insert(r, i);
                    included = true;
                    break;
                } else if room[i] != r + 1 {
                    if let Some(k) = room.iter().position(|&a| a != 0) {
                        ready_out.insert(r, k);
                        included = true;
                    }
                    break;
                }
            }
            if !included {
                full_rooms.insert(r);
            }
        }
        State {
            rooms,
            hall,
            ready_in,
            ready_out,
            full_rooms,
        }
    }

    fn move2hall(&mut self, room: usize, hall: usize) -> u32 {
        if hall % 2 == 0 && hall % (2 * self.rooms.len() + 2) != 0 {
            return 0;
        }
        let mut eng;
        let new_f;
        if let Some(f_id) = self.ready_out.get(&room) {
            let (take, skip) = if hall < 2 + 2 * room {
                (2 + 2 * room, hall)
            } else {
                (hall + 1, 3 + 2 * room)
            };
            if self.hall.iter().take(take).skip(skip).any(|&a| a != 0) {
                return 0;
            }
            let a = self.rooms[room][*f_id];
            self.hall[hall] = a;
            self.rooms[room][*f_id] = 0;
            eng = (take - skip + *f_id + 1) as u32 * MULT[a - 1];
            new_f = *f_id + 1;
        } else {
            return 0;
        }
        if self.rooms[room].iter().skip(new_f).all(|&a| a == room + 1) {
            self.ready_out.remove(&room);
            self.ready_in.insert(room, new_f - 1);
        } else {
            self.ready_out.insert(room, new_f);
        }

        while !self.ready_in.is_empty() {
            let mut moved = false;
            // Rooms which can be moved from ready_out to ready_in
            let mut ready_rooms = HashMap::new();
            for (&room, f_id) in self.ready_in.iter_mut() {
                // Check if somebody from the hall on the left can move in
                for h in (0..2 + 2 * room).rev() {
                    if self.hall[h] == room + 1 {
                        moved = true;
                        self.rooms[room][*f_id] = room + 1;
                        self.hall[h] = 0;
                        eng += ((2 + 2 * room - h + *f_id + 1) as u32) * MULT[room];
                        if *f_id > 0 {
                            *f_id -= 1;
                        } else {
                            self.full_rooms.insert(room);
                            break;
                        }
                    } else if self.hall[h] != 0 {
                        break;
                    }
                }
                // The room is full, can move to the next
                if self.full_rooms.contains(&room) {
                    continue;
                }
                // Check if somebody from the hall on the right can move in
                for h in 3 + 2 * room..self.hall.len() {
                    if self.hall[h] == room + 1 {
                        moved = true;
                        self.rooms[room][*f_id] = room + 1;
                        self.hall[h] = 0;
                        eng += ((h - 2 - 2 * room + *f_id + 1) as u32) * MULT[room];
                        if *f_id > 0 {
                            *f_id -= 1;
                        } else {
                            self.full_rooms.insert(room);
                            break;
                        }
                    } else if self.hall[h] != 0 {
                        break;
                    }
                }
                // The room is full, can move to the next
                if self.full_rooms.contains(&room) {
                    continue;
                }
                // Check if somebody from another room can move in
                for (&r, f) in self.ready_out.iter_mut() {
                    // Check the hall in between
                    let (t, s) = if r < room { (room, r) } else { (r, room) };
                    if self
                        .hall
                        .iter()
                        .take(2 * t + 2)
                        .skip(2 * s + 3)
                        .any(|a| *a != 0)
                    {
                        continue;
                    }
                    // Move all possible occupants
                    while *f < self.rooms[r].len() && self.rooms[r][*f] == room + 1 {
                        moved = true;
                        self.rooms[room][*f_id] = room + 1;
                        self.rooms[r][*f] = 0;
                        eng += (((t - s) * 2 + 2 + *f + *f_id) as u32) * MULT[room];
                        *f += 1;
                        if *f_id > 0 {
                            *f_id -= 1;
                        } else {
                            self.full_rooms.insert(room);
                            break;
                        }
                    }
                    // Check if the moved out room can be moved in now
                    if self.rooms[r].iter().skip(*f).all(|&a| a == r + 1) {
                        ready_rooms.insert(r, *f - 1);
                    }
                }
                // Update not_ready room set
                for room in ready_rooms.keys() {
                    self.ready_out.remove(room);
                }
            }
            for room in &self.full_rooms {
                self.ready_in.remove(room);
            }
            self.ready_in.extend(ready_rooms);
            // Finish if the state didn't change
            if !moved {
                break;
            }
        }
        eng
    }

    fn finished(&self) -> bool {
        self.full_rooms.len() == self.rooms.len()
    }

    #[cfg(feature = "solution")]
    fn display(&self) {
        println!("#{}#", self.hall.iter().map(|_| '#').collect::<String>());
        println!(
            "#{}#",
            self.hall.iter().map(|&v| DISP[v]).collect::<String>()
        );
        let edge = (self.hall.len() - 2 * self.rooms.len() + 1) / 2;
        for i in 0..=self.rooms[0].len() {
            let border = vec![if i == 0 { "#" } else { " " }; edge].join("");
            println!(
                "{}#{}#{}",
                border,
                self.rooms
                    .iter()
                    .map(|r| if i == self.rooms[0].len() {
                        "#".to_string()
                    } else {
                        DISP[r[i]].to_string()
                    })
                    .collect::<Vec<String>>()
                    .join("#"),
                border
            );
        }
    }
}

fn count(rooms: &[Vec<usize>]) -> u32 {
    let mut energy = u32::MAX;
    let mut stack = vec![(0, 0, 0, State::new(&rooms, &vec![0; 2 * rooms.len() + 3]))];
    #[cfg(feature = "solution")]
    let mut solution: Vec<(u32, State)> = vec![];

    while !stack.is_empty() {
        let (mut eng, mut room, mut hall, mut state) = stack.pop().unwrap();
        let prev = (eng.to_owned(), state.to_owned());
        while room < state.rooms.len() {
            while hall < state.hall.len() {
                let curr = state.move2hall(room, hall);
                if curr > 0 {
                    eng += curr;
                    break;
                }
                hall += 1;
            }
            if eng > prev.0 {
                break;
            }
            hall = 0;
            room += 1;
        }

        if eng > prev.0 {
            if state.finished() {
                if eng < energy {
                    energy = eng;
                    #[cfg(feature = "solution")]
                    {
                        solution = stack
                            .iter()
                            .map(|(e, _, _, s)| (e.to_owned(), s.to_owned()))
                            .collect();
                        solution.push(prev);
                        solution.push((eng.to_owned(), state.to_owned()));
                    }
                    print!("Reducing solution to {}  \r", eng);
                    io::stdout().flush().unwrap();
                }
            } else {
                stack.push((prev.0, room, hall + 1, prev.1));
                if eng < energy {
                    stack.push((eng, 0, 0, state));
                }
            }
        }
    }
    print!("                                   \r");
    #[cfg(feature = "solution")]
    for (e, s) in solution {
        println!("{}                          ", e);
        s.display();
    }
    energy
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let mut data: Vec<Vec<usize>> = input
        .iter()
        .skip(2)
        .take(2)
        .map(|l| {
            l.chars()
                .filter_map(|c| DISP.iter().position(|&r| r == c))
                .collect()
        })
        .collect();
    if let Step::Second = step {
        data.insert(1, vec![4, 3, 2, 1]);
        data.insert(2, vec![4, 2, 1, 3]);
    }
    let rooms: Vec<Vec<usize>> = (0..data[0].len())
        .map(|i| (0..data.len()).map(|j| data[j][i]).collect())
        .collect();
    count(&rooms).to_string()
}
