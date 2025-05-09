#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AOZContentBriefingBNpc {
exd: EXD,
exh: EXH,
}
impl AOZContentBriefingBNpc {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AOZContentBriefingBNpc").unwrap();let exd = game_data.read_excel_sheet("AOZContentBriefingBNpc", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AOZContentBriefingBNpcRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AOZContentBriefingBNpcRow { columns: row.columns.clone() }
}
}
pub struct AOZContentBriefingBNpcRow {
columns: Vec<ColumnData>,
}
impl AOZContentBriefingBNpcRow {
pub fn BNpcName(&self) -> &ColumnData {
&self.columns[0]
}
pub fn TargetSmall(&self) -> &ColumnData {
&self.columns[1]
}
pub fn TargetLarge(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Endurance(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Fire(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Ice(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Wind(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Earth(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Thunder(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Water(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Slashing(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Piercing(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Blunt(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Magic(&self) -> &ColumnData {
&self.columns[13]
}
pub fn HideStats(&self) -> &ColumnData {
&self.columns[14]
}
pub fn SlowVuln(&self) -> &ColumnData {
&self.columns[15]
}
pub fn PetrificationVuln(&self) -> &ColumnData {
&self.columns[16]
}
pub fn ParalysisVuln(&self) -> &ColumnData {
&self.columns[17]
}
pub fn InterruptionVuln(&self) -> &ColumnData {
&self.columns[18]
}
pub fn BlindVuln(&self) -> &ColumnData {
&self.columns[19]
}
pub fn StunVuln(&self) -> &ColumnData {
&self.columns[20]
}
pub fn SleepVuln(&self) -> &ColumnData {
&self.columns[21]
}
pub fn BindVuln(&self) -> &ColumnData {
&self.columns[22]
}
pub fn HeavyVuln(&self) -> &ColumnData {
&self.columns[23]
}
pub fn FlatOrDeathVuln(&self) -> &ColumnData {
&self.columns[24]
}
}
