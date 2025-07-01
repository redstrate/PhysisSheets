#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct PermissionSheet {
exd: EXD,
exh: EXH,
}
impl PermissionSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("Permission")?;let exd = game_data.read_excel_sheet("Permission", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<PermissionRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(PermissionRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<PermissionRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<PermissionRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct PermissionRow {
columns: Vec<ColumnData>,
}
impl PermissionRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Unknown11(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown14(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Unknown15(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Unknown16(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Unknown17(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Unknown18(&self) -> &ColumnData {
&self.columns[18]
}
pub fn Unknown19(&self) -> &ColumnData {
&self.columns[19]
}
pub fn Unknown20(&self) -> &ColumnData {
&self.columns[20]
}
pub fn Unknown21(&self) -> &ColumnData {
&self.columns[21]
}
pub fn Unknown22(&self) -> &ColumnData {
&self.columns[22]
}
pub fn Unknown23(&self) -> &ColumnData {
&self.columns[23]
}
pub fn Unknown24(&self) -> &ColumnData {
&self.columns[24]
}
pub fn Unknown25(&self) -> &ColumnData {
&self.columns[25]
}
pub fn Unknown26(&self) -> &ColumnData {
&self.columns[26]
}
pub fn Unknown27(&self) -> &ColumnData {
&self.columns[27]
}
pub fn Unknown28(&self) -> &ColumnData {
&self.columns[28]
}
pub fn Unknown29(&self) -> &ColumnData {
&self.columns[29]
}
pub fn Unknown30(&self) -> &ColumnData {
&self.columns[30]
}
pub fn Unknown31(&self) -> &ColumnData {
&self.columns[31]
}
pub fn Unknown32(&self) -> &ColumnData {
&self.columns[32]
}
pub fn Unknown33(&self) -> &ColumnData {
&self.columns[33]
}
pub fn Unknown34(&self) -> &ColumnData {
&self.columns[34]
}
pub fn Unknown35(&self) -> &ColumnData {
&self.columns[35]
}
pub fn Unknown36(&self) -> &ColumnData {
&self.columns[36]
}
pub fn Unknown37(&self) -> &ColumnData {
&self.columns[37]
}
pub fn Unknown38(&self) -> &ColumnData {
&self.columns[38]
}
pub fn Unknown39(&self) -> &ColumnData {
&self.columns[39]
}
pub fn Unknown40(&self) -> &ColumnData {
&self.columns[40]
}
pub fn Unknown41(&self) -> &ColumnData {
&self.columns[41]
}
pub fn Unknown42(&self) -> &ColumnData {
&self.columns[42]
}
pub fn Unknown43(&self) -> &ColumnData {
&self.columns[43]
}
pub fn Unknown44(&self) -> &ColumnData {
&self.columns[44]
}
pub fn Unknown45(&self) -> &ColumnData {
&self.columns[45]
}
pub fn Unknown46(&self) -> &ColumnData {
&self.columns[46]
}
pub fn Unknown47(&self) -> &ColumnData {
&self.columns[47]
}
pub fn Unknown48(&self) -> &ColumnData {
&self.columns[48]
}
pub fn Unknown49(&self) -> &ColumnData {
&self.columns[49]
}
pub fn Unknown50(&self) -> &ColumnData {
&self.columns[50]
}
pub fn Unknown51(&self) -> &ColumnData {
&self.columns[51]
}
pub fn Unknown52(&self) -> &ColumnData {
&self.columns[52]
}
pub fn Unknown53(&self) -> &ColumnData {
&self.columns[53]
}
pub fn Unknown54(&self) -> &ColumnData {
&self.columns[54]
}
pub fn Unknown55(&self) -> &ColumnData {
&self.columns[55]
}
pub fn Unknown56(&self) -> &ColumnData {
&self.columns[56]
}
pub fn Unknown57(&self) -> &ColumnData {
&self.columns[57]
}
pub fn Unknown58(&self) -> &ColumnData {
&self.columns[58]
}
pub fn Unknown59(&self) -> &ColumnData {
&self.columns[59]
}
pub fn Unknown60(&self) -> &ColumnData {
&self.columns[60]
}
pub fn Unknown61(&self) -> &ColumnData {
&self.columns[61]
}
pub fn Unknown62(&self) -> &ColumnData {
&self.columns[62]
}
pub fn Unknown63(&self) -> &ColumnData {
&self.columns[63]
}
pub fn Unknown64(&self) -> &ColumnData {
&self.columns[64]
}
pub fn Unknown65(&self) -> &ColumnData {
&self.columns[65]
}
pub fn Unknown66(&self) -> &ColumnData {
&self.columns[66]
}
pub fn Unknown67(&self) -> &ColumnData {
&self.columns[67]
}
pub fn Unknown68(&self) -> &ColumnData {
&self.columns[68]
}
pub fn Unknown69(&self) -> &ColumnData {
&self.columns[69]
}
pub fn Unknown70(&self) -> &ColumnData {
&self.columns[70]
}
pub fn Unknown71(&self) -> &ColumnData {
&self.columns[71]
}
pub fn Unknown72(&self) -> &ColumnData {
&self.columns[72]
}
pub fn Unknown73(&self) -> &ColumnData {
&self.columns[73]
}
pub fn Unknown74(&self) -> &ColumnData {
&self.columns[74]
}
pub fn Unknown75(&self) -> &ColumnData {
&self.columns[75]
}
pub fn Unknown76(&self) -> &ColumnData {
&self.columns[76]
}
pub fn Unknown77(&self) -> &ColumnData {
&self.columns[77]
}
pub fn Unknown78(&self) -> &ColumnData {
&self.columns[78]
}
pub fn Unknown79(&self) -> &ColumnData {
&self.columns[79]
}
pub fn Unknown80(&self) -> &ColumnData {
&self.columns[80]
}
pub fn Unknown81(&self) -> &ColumnData {
&self.columns[81]
}
pub fn Unknown82(&self) -> &ColumnData {
&self.columns[82]
}
pub fn Unknown83(&self) -> &ColumnData {
&self.columns[83]
}
pub fn Unknown84(&self) -> &ColumnData {
&self.columns[84]
}
pub fn Unknown85(&self) -> &ColumnData {
&self.columns[85]
}
pub fn Unknown86(&self) -> &ColumnData {
&self.columns[86]
}
pub fn Unknown87(&self) -> &ColumnData {
&self.columns[87]
}
pub fn Unknown88(&self) -> &ColumnData {
&self.columns[88]
}
pub fn Unknown89(&self) -> &ColumnData {
&self.columns[89]
}
pub fn Unknown90(&self) -> &ColumnData {
&self.columns[90]
}
pub fn Unknown91(&self) -> &ColumnData {
&self.columns[91]
}
pub fn Unknown92(&self) -> &ColumnData {
&self.columns[92]
}
pub fn Unknown93(&self) -> &ColumnData {
&self.columns[93]
}
pub fn Unknown94(&self) -> &ColumnData {
&self.columns[94]
}
pub fn Unknown95(&self) -> &ColumnData {
&self.columns[95]
}
pub fn Unknown96(&self) -> &ColumnData {
&self.columns[96]
}
pub fn Unknown97(&self) -> &ColumnData {
&self.columns[97]
}
pub fn Unknown98(&self) -> &ColumnData {
&self.columns[98]
}
pub fn Unknown99(&self) -> &ColumnData {
&self.columns[99]
}
pub fn Unknown100(&self) -> &ColumnData {
&self.columns[100]
}
pub fn Unknown101(&self) -> &ColumnData {
&self.columns[101]
}
pub fn Unknown102(&self) -> &ColumnData {
&self.columns[102]
}
pub fn Unknown103(&self) -> &ColumnData {
&self.columns[103]
}
}
