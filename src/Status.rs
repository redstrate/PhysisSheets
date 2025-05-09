#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct StatusSheet {
exd: EXD,
exh: EXH,
}
impl StatusSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("Status")?;let exd = game_data.read_excel_sheet("Status", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<StatusRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(StatusRow { columns })
}
}
pub struct StatusRow {
columns: Vec<ColumnData>,
}
impl StatusRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ParamModifier(&self) -> &ColumnData {
&self.columns[3]
}
pub fn VFX(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Log(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[6]
}
pub fn MaxStacks(&self) -> &ColumnData {
&self.columns[7]
}
pub fn ClassJobCategory(&self) -> &ColumnData {
&self.columns[8]
}
pub fn StatusCategory(&self) -> &ColumnData {
&self.columns[9]
}
pub fn HitEffect(&self) -> &ColumnData {
&self.columns[10]
}
pub fn PartyListPriority(&self) -> &ColumnData {
&self.columns[11]
}
pub fn CanIncreaseRewards(&self) -> &ColumnData {
&self.columns[12]
}
pub fn ParamEffect(&self) -> &ColumnData {
&self.columns[13]
}
pub fn TargetType(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Flags(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Flag2(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Unknown_70_1(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[18]
}
pub fn LockMovement(&self) -> &ColumnData {
&self.columns[19]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[20]
}
pub fn LockActions(&self) -> &ColumnData {
&self.columns[21]
}
pub fn LockControl(&self) -> &ColumnData {
&self.columns[22]
}
pub fn Transfiguration(&self) -> &ColumnData {
&self.columns[23]
}
pub fn IsGaze(&self) -> &ColumnData {
&self.columns[24]
}
pub fn CanDispel(&self) -> &ColumnData {
&self.columns[25]
}
pub fn InflictedByActor(&self) -> &ColumnData {
&self.columns[26]
}
pub fn IsPermanent(&self) -> &ColumnData {
&self.columns[27]
}
pub fn NoLogVfx(&self) -> &ColumnData {
&self.columns[28]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[29]
}
pub fn CanStatusOff(&self) -> &ColumnData {
&self.columns[30]
}
pub fn IsFcBuff(&self) -> &ColumnData {
&self.columns[31]
}
pub fn Invisibility(&self) -> &ColumnData {
&self.columns[32]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[33]
}
pub fn Unknown_70_2(&self) -> &ColumnData {
&self.columns[34]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[35]
}
}
