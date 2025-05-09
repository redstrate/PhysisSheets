#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Transformation {
exd: EXD,
exh: EXH,
}
impl Transformation {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Transformation").unwrap();let exd = game_data.read_excel_sheet("Transformation", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TransformationRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TransformationRow { columns: row.columns.clone() }
}
}
pub struct TransformationRow {
columns: Vec<ColumnData>,
}
impl TransformationRow {
pub fn Speed(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Scale(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Action6(&self) -> &ColumnData {
&self.columns[2]
}
pub fn BNpcCustomize(&self) -> &ColumnData {
&self.columns[3]
}
pub fn NpcEquip(&self) -> &ColumnData {
&self.columns[4]
}
pub fn BNpcName(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Action0(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Action1(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Action2(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Action3(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Action4(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Action5(&self) -> &ColumnData {
&self.columns[11]
}
pub fn RPParameter(&self) -> &ColumnData {
&self.columns[12]
}
pub fn RemoveAction(&self) -> &ColumnData {
&self.columns[13]
}
pub fn StartVFX(&self) -> &ColumnData {
&self.columns[14]
}
pub fn EndVFX(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Action7(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Model(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[18]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[19]
}
pub fn RPParameter2(&self) -> &ColumnData {
&self.columns[20]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[21]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[22]
}
pub fn ExHotbarEnableConfig(&self) -> &ColumnData {
&self.columns[23]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[24]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[25]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[26]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[27]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[28]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[29]
}
pub fn Unknown11(&self) -> &ColumnData {
&self.columns[30]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[31]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[32]
}
pub fn IsPvP(&self) -> &ColumnData {
&self.columns[33]
}
pub fn IsEvent(&self) -> &ColumnData {
&self.columns[34]
}
pub fn PlayerCamera(&self) -> &ColumnData {
&self.columns[35]
}
pub fn Unknown14(&self) -> &ColumnData {
&self.columns[36]
}
pub fn Unknown15(&self) -> &ColumnData {
&self.columns[37]
}
pub fn Unknown16(&self) -> &ColumnData {
&self.columns[38]
}
pub fn Unknown17(&self) -> &ColumnData {
&self.columns[39]
}
}
