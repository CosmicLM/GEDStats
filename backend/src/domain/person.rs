// step 1, identify the ID of the person
pub struct PersonStats {
    pub given_name: String,
    pub surname: String,
    pub gender: String,
    pub id: String,
}
    //step 1.1, what would be ID on .ged file?
    //step 1.2, gedcom.rs parses the id
// step 2, sort the name into distinct boxes, Given name, Surname, Prefix and SUffix
// step 3, identify aproximate dates, and display raw text instead of the factual number
// step 4, implement boxes for male, female and unknoen
// step 5, connections to family, parents, spouses and children