use std::io;
use std::cmp;

use std::io::BufRead;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

type DamagePoints = u32;
type GroupId = usize;

#[derive(Debug, Clone)]
enum ParseGroupError{
    InvalidFormat(String),
    InvalidDamageType(String),
    InvalidNumber(String),
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
enum DamageType {
    Slashing,
    Cold,
    Radiation,
    Bludgeoning,
    Fire,
}
impl FromStr for DamageType {
    type Err = ParseGroupError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "slashing" => Ok(DamageType::Slashing),
            "cold" => Ok(DamageType::Cold),
            "radiation" => Ok(DamageType::Radiation),
            "bludgeoning" => Ok(DamageType::Bludgeoning),
            "fire" => Ok(DamageType::Fire),
            _ => Err(ParseGroupError::InvalidDamageType(s.to_string())),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
enum GroupType {
    ImmuneSystem,
    Infection
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Group {
    group_type: GroupType,
    num_of_units: usize,
    hit_points: DamagePoints,
    weak_to: HashSet<DamageType>,
    immune_to: HashSet<DamageType>,
    damage_type: DamageType,
    damage: DamagePoints,
    initiative: usize,
}
impl Group {
    fn get_effective_power(&self) -> DamagePoints {
        self.num_of_units as DamagePoints * self.damage
    }
    fn deal_damage_by(&mut self, group: &Self) {
        let damage = group.estimate_damage(self);
        let num_of_killed_units = (damage / self.hit_points) as usize;
        if num_of_killed_units < self.num_of_units {
            self.num_of_units -= num_of_killed_units;
        } else {
            self.num_of_units = 0;
        }
    }
    fn parse_int<T: FromStr>(s: &str) -> Result<T, ParseGroupError> {
        match s.parse() {
            Ok(m) => Ok(m),
            Err(_) => Err(ParseGroupError::InvalidNumber(s.to_string())),
        }
    }
    fn estimate_damage(&self, group: &Self) -> DamagePoints {
        let effective_power = self.get_effective_power();
        if group.immune_to.contains(&self.damage_type) {
            return 0;
        }
        if group.weak_to.contains(&self.damage_type) {
            return 2 * effective_power;
        }
        return effective_power;
    }
}
impl cmp::Ord for Group {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_effective_power().cmp(&other.get_effective_power()).reverse()
            .then(self.initiative.cmp(&other.initiative).reverse())
    }
}
impl cmp::PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn extract_damage_types(results: &Vec<Result<DamageType, ParseGroupError>>) -> Result<Vec<DamageType>, ParseGroupError> {
    let mut damage_types = Vec::new();
    for r in results {
        match r {
            Ok(dt) => {
                damage_types.push(dt.clone());
            }
            Err(err) => {
                return Err(err.clone());
            }
        }
    }
    return Ok(damage_types);
}

impl FromStr for Group {
    type Err = ParseGroupError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let segments: Vec<_> = s.split(|c| c == '(' || c == ')').collect();

        let unit_hp: String;
        let weak_immune: String;
        let damage_info: String;

        if segments.len() == 3 {
            unit_hp = segments[0].trim().to_string();
            weak_immune = segments[1].trim().to_string();
            damage_info = segments[2].trim().to_string();
        } else if segments.len() == 1 {
            let spaced_segments: Vec<_> = segments[0].trim().split(' ').collect();
            unit_hp = spaced_segments.iter().take(7).cloned().collect::<Vec<&str>>().join(" ");
            weak_immune = String::from("");
            damage_info = spaced_segments.iter().skip(7).cloned().collect::<Vec<&str>>().join(" ");
        } else {
            return Err(ParseGroupError::InvalidFormat(s.to_string()));
        }

        let unit_hp_segments: Vec<_> = unit_hp.split(' ').collect();
        let weak_immune_segments: Vec<_> = weak_immune.split(';').collect();
        let damage_info_segments: Vec<_> = damage_info.split(' ').collect();
        let mut weak_immune_map: HashMap<&str, Vec<DamageType>> = HashMap::new();

        for seg in weak_immune_segments {
            let spaced_segments: Vec<_> = seg
                .trim()
                .split(' ')
                .collect();

            let results: Vec<_> = spaced_segments
                .iter()
                .skip(2)
                .map(|el| el.trim_end_matches(','))
                .map(|el| Self::parse_int(el))
                .collect();
            let damage_types = extract_damage_types(&results)?;
            weak_immune_map.insert(spaced_segments[0], damage_types);
        }

        if unit_hp_segments.len() != 7 {
            return Err(ParseGroupError::InvalidFormat(unit_hp.to_string()));
        }
        if damage_info_segments.len() != 11 {
            return Err(ParseGroupError::InvalidFormat(damage_info.to_string()));
        }
        let num_of_units: usize = Self::parse_int(unit_hp_segments[0])?;
        let hit_points: DamagePoints = Self::parse_int(unit_hp_segments[4])?;

        let immune_to: HashSet<DamageType> = weak_immune_map.remove("immune").unwrap_or(Vec::new())
            .iter().cloned().collect();
        let weak_to: HashSet<DamageType> = weak_immune_map.remove("weak").unwrap_or(Vec::new())
            .iter().cloned().collect();

        let damage: DamagePoints = Self::parse_int(damage_info_segments[5])?;
        let damage_type: DamageType = damage_info_segments[6].parse()?;
        let initiative: usize = Self::parse_int(damage_info_segments[10])?;

        return Ok(Group {
            group_type: GroupType::ImmuneSystem,
            num_of_units,
            hit_points,
            weak_to,
            immune_to,
            damage,
            damage_type,
            initiative,
        });
    }
}

fn argmax<T: Clone, S: cmp::Ord>(vec: &Vec<T>, f: impl Fn(&T) -> S) -> Option<T> {
    let mut best_el: Option<&T> = None;

    for el in vec {
        let val = f(el);
        match best_el.map(|el| f(el)) {
            None => {
                best_el = Some(el);
            },
            Some(bv) => {
                match bv.cmp(&val) {
                    Ordering::Less => {
                        best_el = Some(el);
                    },
                    _ => {},
                }
            },
        }
    }

    return best_el.cloned();
}

fn read_input() -> Vec<Group> {
    let stdin = io::stdin();
    let mut group_type = GroupType::ImmuneSystem;
    let mut groups = Vec::new();
    let mut id_counter = 0;
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.len() == 0 {
            continue;
        }
        else if l == "Immune System:" {
            group_type = GroupType::ImmuneSystem;
        }
        else if l == "Infection:" {
            group_type = GroupType::Infection;
        }
        else {
            let mut g: Group = l.parse().expect("invalid group");
            g.group_type = group_type.clone();
            groups.push(g);
            id_counter += 1;
        }
    }
    return groups;
}

fn get_group_types(groups_map: &HashMap<GroupId, Group>) -> HashSet<GroupType> {
    let mut group_types: HashSet<GroupType> = HashSet::new();
    for g in groups_map.values() {
        group_types.insert(g.group_type.clone());
    }
    return group_types;
}

fn trace_groups(groups_map: &HashMap<GroupId, Group>) {
    for (gid, g) in groups_map {
        println!("Group {}: {:?}", gid, g);
    }
}

fn main() {
    let initial_groups = read_input();
    let mut groups_map: HashMap<GroupId, Group> = initial_groups.into_iter().enumerate().collect();
    while get_group_types(&groups_map).len() > 1 {
        println!("-- Next turn --");
        trace_groups(&groups_map);
        let mut group_items: Vec<(GroupId, Group)> = Vec::new();
        for (k,v) in &groups_map {
            group_items.push((*k, v.clone()));
        }

        group_items.sort_by(|(_, a), (_, b)| a.get_effective_power().cmp(&b.get_effective_power()).reverse().then(a.initiative.cmp(&b.initiative).reverse()));

        let mut selected_ids_map: HashMap<GroupId, GroupId> = HashMap::new();
        let mut attacked_ids: HashSet<GroupId> = HashSet::new();
        let mut attack_sequence: Vec<(GroupId, GroupId)> = Vec::new();

        for (group_id, group) in &group_items {
            let selection_opt = argmax(
                &group_items.iter()
                    .filter(|(_, g)| g.group_type != group.group_type)
                    .filter(|(i, _)| !attacked_ids.contains(&i))
                    .collect::<Vec<&(GroupId, Group)>>(),
                |(_, g)| group.estimate_damage(g));

            if selection_opt.is_none() {
                continue;
            }

            let (sel_group_id, sel_group) = selection_opt.unwrap();

            selected_ids_map.insert(*group_id, *sel_group_id);
            attacked_ids.insert(*sel_group_id);
        }

        group_items.sort_by(|(_, a), (_, b)| a.initiative.cmp(&b.initiative).reverse());

        for (group_id, group) in &group_items {
            let selected_group_id_opt = selected_ids_map.get(&group_id);
            if selected_group_id_opt.is_none() {
                continue;
            }
            let selected_group_id: GroupId = selected_group_id_opt.unwrap().clone();
            attack_sequence.push((*group_id, selected_group_id));
        }

        for (group_id, selected_group_id) in attack_sequence {
            let damaging_group: Group = groups_map.get(&group_id).unwrap().clone();
            let group: &mut Group = groups_map.get_mut(&selected_group_id).unwrap();
            let num_of_units_before = group.num_of_units;
            group.deal_damage_by(&damaging_group);
            let num_of_units_after = group.num_of_units;

            println!("Group {} attacks group {}, killing {} units", group_id, selected_group_id, num_of_units_before - num_of_units_after);
        }

        let eliminated_ids: Vec<_> = groups_map.iter().filter(|(_,g)| g.num_of_units == 0).map(|(gid,_)| gid).cloned().collect();

        for id in eliminated_ids {
            println!("Group {} eliminated", id);
            groups_map.remove(&id);
        }
    }

    println!("-- finish --");
    trace_groups(&groups_map);

    let num_of_winning_units = groups_map.values().map(|g| g.num_of_units).fold(0, |n, el| n + el);

    println!("Winning number of units: {}", num_of_winning_units);
}
