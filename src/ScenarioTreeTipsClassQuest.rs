#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ScenarioTreeTipsClassQuest {
exd: EXD,
exh: EXH,
}
impl ScenarioTreeTipsClassQuest {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ScenarioTreeTipsClassQuest").unwrap();let exd = game_data.read_excel_sheet("ScenarioTreeTipsClassQuest", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ScenarioTreeTipsClassQuestRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ScenarioTreeTipsClassQuestRow { columns: row.columns.clone() }
}
}
pub struct ScenarioTreeTipsClassQuestRow {
columns: Vec<ColumnData>,
}
impl ScenarioTreeTipsClassQuestRow {
pub fn Quest(&self) -> &ColumnData {
&self.columns[0]
}
pub fn RequiredQuest(&self) -> &ColumnData {
&self.columns[1]
}
pub fn RequiredLevel(&self) -> &ColumnData {
&self.columns[2]
}
pub fn RequiredExpansion(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[5]
}
}
