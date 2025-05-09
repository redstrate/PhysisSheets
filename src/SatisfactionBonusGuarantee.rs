#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct SatisfactionBonusGuarantee {
exd: EXD,
exh: EXH,
}
impl SatisfactionBonusGuarantee {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("SatisfactionBonusGuarantee").unwrap();let exd = game_data.read_excel_sheet("SatisfactionBonusGuarantee", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> SatisfactionBonusGuaranteeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
SatisfactionBonusGuaranteeRow { columns: row.columns.clone() }
}
}
pub struct SatisfactionBonusGuaranteeRow {
columns: Vec<ColumnData>,
}
impl SatisfactionBonusGuaranteeRow {
pub fn BonusDoH(&self) -> &ColumnData {
&self.columns[0]
}
pub fn BonusDoL(&self) -> &ColumnData {
&self.columns[1]
}
pub fn BonusFisher(&self) -> &ColumnData {
&self.columns[2]
}
}
