// step 1, identify the ID of the person
    //step 1.2, gedcom.rs parses the id
  
pub struct PersonStats {
    pub given_name: String,
    pub surname: String,
    pub enum gender {
        Male,
        Female,
        Unknown,
    } 
    pub id: String,

 let result = gedcom::parse_file();

 match result{
    Ok(data) => {
        for individual in data.individuals():
        let current_id = individual.xref.to_string();

        let Some(name) = individual.name().next() else { continue };
        let surname = name.next() else { continue };


 }

}
 Err(_) => {
    println!("No parse file")
 }

     }
     
     
     //locate string with 0
    //idenfity the @ symbols
    //confirmation of INDI (individual)


// step 2, sort the name into distinct boxes, Given name, Surname, Prefix and SUffix
// step 3, identify aproximate dates, and display raw text instead of the factual number
// step 4, implement boxes for male, female and unknoen
// step 5, connections to family, parents, spouses and children