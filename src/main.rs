use crate::types::person::{Person, Preference};
use crate::types::timetable::{Timetable, TurnHours, Turn, Day};

mod types;

fn main() {
    let mut timetable = Timetable::new(); //load default values

    let mut people = Vec::with_capacity(7usize);

    people.push(Person::with_preferences("Andrea", "Bonvissuto", vec![
        Preference::new(Day::Mon, Turn::Morning),
        Preference::new(Day::Tue, Turn::Morning),
        Preference::new(Day::Wed, Turn::Morning),
        Preference::new(Day::Thu, Turn::Morning),
        Preference::new(Day::Fri, Turn::Morning),
        //afternoon
        Preference::new(Day::Mon, Turn::Afternoon),
        // Preference::new(Day::Tue, Turn::Afternoon),
        // Preference::new(Day::Wed, Turn::Afternoon),
        Preference::new(Day::Thu, Turn::Afternoon),
        // Preference::new(Day::Fri, Turn::Afternoon),
    ], 0.0));
    people.push(Person::with_preferences("Niccolò", "Querini Squillari", vec![
        // Preference::new(Day::Mon, Turn::Morning),
        // Preference::new(Day::Tue, Turn::Morning),
        // Preference::new(Day::Wed, Turn::Morning),
        // Preference::new(Day::Thu, Turn::Morning),
        // Preference::new(Day::Fri, Turn::Morning),
        //afternoon
        Preference::new(Day::Mon, Turn::Afternoon),
        // Preference::new(Day::Tue, Turn::Afternoon),
        // Preference::new(Day::Wed, Turn::Afternoon),
        // Preference::new(Day::Thu, Turn::Afternoon),
        // Preference::new(Day::Fri, Turn::Afternoon),
    ], 0.0));
    people.push(Person::with_preferences("Domenico", "Elia", vec![
        Preference::new(Day::Mon, Turn::Morning),
        Preference::new(Day::Tue, Turn::Morning),
        // Preference::new(Day::Wed, Turn::Morning),
        Preference::new(Day::Thu, Turn::Morning),
        // Preference::new(Day::Fri, Turn::Morning),
        //afternoon
        // Preference::new(Day::Mon, Turn::Afternoon),
        Preference::new(Day::Tue, Turn::Afternoon),
        Preference::new(Day::Wed, Turn::Afternoon),
        Preference::new(Day::Thu, Turn::Afternoon),
        Preference::new(Day::Fri, Turn::Afternoon),
    ], 0.0));
    people.push(Person::with_preferences("Giovanni", "Giunta", vec![
        // Preference::new(Day::Mon, Turn::Morning),
        Preference::new(Day::Tue, Turn::Morning),
        // Preference::new(Day::Wed, Turn::Morning),
        Preference::new(Day::Thu, Turn::Morning),
        // Preference::new(Day::Fri, Turn::Morning),
        //afternoon
        // Preference::new(Day::Mon, Turn::Afternoon),
        Preference::new(Day::Tue, Turn::Afternoon),
        // Preference::new(Day::Wed, Turn::Afternoon),
        // Preference::new(Day::Thu, Turn::Afternoon),
        Preference::new(Day::Fri, Turn::Afternoon),
    ], 0.0));
    people.push(Person::with_preferences("Daniele", "De Rossi", vec![
        // Preference::new(Day::Mon, Turn::Morning),
        Preference::new(Day::Tue, Turn::Morning),
        // Preference::new(Day::Wed, Turn::Morning),
        Preference::new(Day::Thu, Turn::Morning),
        // Preference::new(Day::Fri, Turn::Morning),
        //afternoon
        // Preference::new(Day::Mon, Turn::Afternoon),
        Preference::new(Day::Tue, Turn::Afternoon),
        // Preference::new(Day::Wed, Turn::Afternoon),
        // Preference::new(Day::Thu, Turn::Afternoon),
        Preference::new(Day::Fri, Turn::Afternoon),
    ], 0.0));
    people.push(Person::with_preferences("Luca", "De Candia", vec![
        // Preference::new(Day::Mon, Turn::Morning),
        // Preference::new(Day::Tue, Turn::Morning),
        Preference::new(Day::Wed, Turn::Morning),
        // Preference::new(Day::Thu, Turn::Morning),
        Preference::new(Day::Fri, Turn::Morning),
        //afternoon
        // Preference::new(Day::Mon, Turn::Afternoon),
        Preference::new(Day::Tue, Turn::Afternoon),
        // Preference::new(Day::Wed, Turn::Afternoon),
        Preference::new(Day::Thu, Turn::Afternoon),
        // Preference::new(Day::Fri, Turn::Afternoon),
    ], 0.0));
    people.push(Person::with_preferences("Vincenzo", "Miccichè", vec![
        // Preference::new(Day::Mon, Turn::Morning),
        Preference::new(Day::Tue, Turn::Morning),
        // Preference::new(Day::Wed, Turn::Morning),
        // Preference::new(Day::Thu, Turn::Morning),
        // Preference::new(Day::Fri, Turn::Morning),
        //afternoon
        // Preference::new(Day::Mon, Turn::Afternoon),
        // Preference::new(Day::Tue, Turn::Afternoon),
        Preference::new(Day::Wed, Turn::Afternoon),
        // Preference::new(Day::Thu, Turn::Afternoon),
        // Preference::new(Day::Fri, Turn::Afternoon),
    ], 0.0));

    timetable.add_people(people);
    //now calc the timetable and print

    timetable.calc();
}
