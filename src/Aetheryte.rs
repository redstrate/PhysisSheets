#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct AetheryteSheet {
exd: EXD,
exh: EXH,
}
impl AetheryteSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("Aetheryte")?;let exd = game_data.read_excel_sheet("Aetheryte", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<AetheryteRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(AetheryteRow { columns })
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
pub fn Level(&self) -> [&ColumnData; 4] {
[&self.columns[9],&self.columns[10],&self.columns[11],&self.columns[12],]
}
pub fn RequiredQuest(&self) -> &ColumnData {
&self.columns[13]
}
pub fn PlaceName(&self) -> &ColumnData {
&self.columns[14]
}
pub fn AethernetName(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Territory(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Map(&self) -> &ColumnData {
&self.columns[17]
}
pub fn AetherstreamX(&self) -> &ColumnData {
&self.columns[18]
}
pub fn AetherstreamY(&self) -> &ColumnData {
&self.columns[19]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[20]
}
pub fn AethernetGroup(&self) -> &ColumnData {
&self.columns[21]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[22]
}
pub fn IsAetheryte(&self) -> &ColumnData {
&self.columns[23]
}
pub fn Invisible(&self) -> &ColumnData {
&self.columns[24]
}
}
