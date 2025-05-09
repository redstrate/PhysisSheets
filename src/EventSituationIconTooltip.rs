#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct EventSituationIconTooltip {
exd: EXD,
exh: EXH,
}
impl EventSituationIconTooltip {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("EventSituationIconTooltip").unwrap();let exd = game_data.read_excel_sheet("EventSituationIconTooltip", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> EventSituationIconTooltipRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
EventSituationIconTooltipRow { columns: row.columns.clone() }
}
}
pub struct EventSituationIconTooltipRow {
columns: Vec<ColumnData>,
}
impl EventSituationIconTooltipRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[1]
}
}
