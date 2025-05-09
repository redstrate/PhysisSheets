#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Aetheryte {
exd: EXD,
exh: EXH,
}
impl Aetheryte {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Aetheryte").unwrap();let exd = game_data.read_excel_sheet("Aetheryte", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AetheryteRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AetheryteRow { columns: row.columns.clone() }
}
}
pub struct AetheryteRow {
columns: Vec<ColumnData>,
}
impl AetheryteRow {
pub fn Singular(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Plural(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Adjective(&self) -> &ColumnData {
&self.columns[2]
}
pub fn PossessivePronoun(&self) -> &ColumnData {
&self.columns[3]
}
pub fn StartsWithVowel(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Pronoun(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Article(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Level(&self) -> &ColumnData {
&self.columns[9]
}
pub fn RequiredQuest(&self) -> &ColumnData {
&self.columns[10]
}
pub fn PlaceName(&self) -> &ColumnData {
&self.columns[11]
}
pub fn AethernetName(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Territory(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Map(&self) -> &ColumnData {
&self.columns[14]
}
pub fn AetherstreamX(&self) -> &ColumnData {
&self.columns[15]
}
pub fn AetherstreamY(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[17]
}
pub fn AethernetGroup(&self) -> &ColumnData {
&self.columns[18]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[19]
}
pub fn IsAetheryte(&self) -> &ColumnData {
&self.columns[20]
}
pub fn Invisible(&self) -> &ColumnData {
&self.columns[21]
}
}
