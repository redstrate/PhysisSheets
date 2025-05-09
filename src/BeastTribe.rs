#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BeastTribe {
exd: EXD,
exh: EXH,
}
impl BeastTribe {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BeastTribe").unwrap();let exd = game_data.read_excel_sheet("BeastTribe", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BeastTribeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BeastTribeRow { columns: row.columns.clone() }
}
}
pub struct BeastTribeRow {
columns: Vec<ColumnData>,
}
impl BeastTribeRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Plural(&self) -> &ColumnData {
&self.columns[1]
}
pub fn NameRelation(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Adjective(&self) -> &ColumnData {
&self.columns[3]
}
pub fn PossessivePronoun(&self) -> &ColumnData {
&self.columns[4]
}
pub fn StartsWithVowel(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Pronoun(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Article(&self) -> &ColumnData {
&self.columns[7]
}
pub fn DEF(&self) -> &ColumnData {
&self.columns[8]
}
pub fn IconReputation(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[11]
}
pub fn CurrencyItem(&self) -> &ColumnData {
&self.columns[12]
}
pub fn MinLevel(&self) -> &ColumnData {
&self.columns[13]
}
pub fn BeastRankBonus(&self) -> &ColumnData {
&self.columns[14]
}
pub fn MaxRank(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Expansion(&self) -> &ColumnData {
&self.columns[16]
}
pub fn DisplayOrder(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[18]
}
}
