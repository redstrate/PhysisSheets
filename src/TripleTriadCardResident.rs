#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct TripleTriadCardResident {
exd: EXD,
exh: EXH,
}
impl TripleTriadCardResident {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("TripleTriadCardResident").unwrap();let exd = game_data.read_excel_sheet("TripleTriadCardResident", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TripleTriadCardResidentRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TripleTriadCardResidentRow { columns: row.columns.clone() }
}
}
pub struct TripleTriadCardResidentRow {
columns: Vec<ColumnData>,
}
impl TripleTriadCardResidentRow {
pub fn Acquisition(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Location(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Quest(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[3]
}
pub fn SaleValue(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Top(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Bottom(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Left(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Right(&self) -> &ColumnData {
&self.columns[9]
}
pub fn TripleTriadCardRarity(&self) -> &ColumnData {
&self.columns[10]
}
pub fn TripleTriadCardType(&self) -> &ColumnData {
&self.columns[11]
}
pub fn SortKey(&self) -> &ColumnData {
&self.columns[12]
}
pub fn UIPriority(&self) -> &ColumnData {
&self.columns[13]
}
pub fn AcquisitionType(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[15]
}
}
