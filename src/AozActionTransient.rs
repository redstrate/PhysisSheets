#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AozActionTransient {
exd: EXD,
exh: EXH,
}
impl AozActionTransient {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AozActionTransient").unwrap();let exd = game_data.read_excel_sheet("AozActionTransient", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AozActionTransientRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AozActionTransientRow { columns: row.columns.clone() }
}
}
pub struct AozActionTransientRow {
columns: Vec<ColumnData>,
}
impl AozActionTransientRow {
pub fn Stats(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[2]
}
pub fn RequiredForQuest(&self) -> &ColumnData {
&self.columns[3]
}
pub fn PreviousQuest(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Location(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Number(&self) -> &ColumnData {
&self.columns[6]
}
pub fn LocationKey(&self) -> &ColumnData {
&self.columns[7]
}
pub fn TargetsEnemy(&self) -> &ColumnData {
&self.columns[8]
}
pub fn TargetsSelfOrAlly(&self) -> &ColumnData {
&self.columns[9]
}
pub fn CauseSlow(&self) -> &ColumnData {
&self.columns[10]
}
pub fn CausePetrify(&self) -> &ColumnData {
&self.columns[11]
}
pub fn CauseParalysis(&self) -> &ColumnData {
&self.columns[12]
}
pub fn CauseInterrupt(&self) -> &ColumnData {
&self.columns[13]
}
pub fn CauseBlind(&self) -> &ColumnData {
&self.columns[14]
}
pub fn CauseStun(&self) -> &ColumnData {
&self.columns[15]
}
pub fn CauseSleep(&self) -> &ColumnData {
&self.columns[16]
}
pub fn CauseBind(&self) -> &ColumnData {
&self.columns[17]
}
pub fn CauseHeavy(&self) -> &ColumnData {
&self.columns[18]
}
pub fn CauseDeath(&self) -> &ColumnData {
&self.columns[19]
}
}
