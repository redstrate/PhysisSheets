#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct RecipeSheet {
exd: EXD,
exh: EXH,
}
impl RecipeSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("Recipe")?;let exd = game_data.read_excel_sheet("Recipe", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<RecipeRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(RecipeRow { columns })
}
}
pub struct RecipeRow {
columns: Vec<ColumnData>,
}
impl RecipeRow {
pub fn RequiredQuality(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Quest(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Number(&self) -> &ColumnData {
&self.columns[2]
}
pub fn CraftType(&self) -> &ColumnData {
&self.columns[3]
}
pub fn ItemResult(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Ingredient(&self) -> [&ColumnData; 8] {
[&self.columns[5],&self.columns[6],&self.columns[7],&self.columns[8],&self.columns[9],&self.columns[10],&self.columns[11],&self.columns[12],]
}
pub fn StatusRequired(&self) -> &ColumnData {
&self.columns[13]
}
pub fn ItemRequired(&self) -> &ColumnData {
&self.columns[14]
}
pub fn RecipeLevelTable(&self) -> &ColumnData {
&self.columns[15]
}
pub fn MaxAdjustableJobLevel(&self) -> &ColumnData {
&self.columns[16]
}
pub fn RecipeNotebookList(&self) -> &ColumnData {
&self.columns[17]
}
pub fn DisplayPriority(&self) -> &ColumnData {
&self.columns[18]
}
pub fn DifficultyFactor(&self) -> &ColumnData {
&self.columns[19]
}
pub fn QualityFactor(&self) -> &ColumnData {
&self.columns[20]
}
pub fn DurabilityFactor(&self) -> &ColumnData {
&self.columns[21]
}
pub fn RequiredCraftsmanship(&self) -> &ColumnData {
&self.columns[22]
}
pub fn RequiredControl(&self) -> &ColumnData {
&self.columns[23]
}
pub fn QuickSynthCraftsmanship(&self) -> &ColumnData {
&self.columns[24]
}
pub fn QuickSynthControl(&self) -> &ColumnData {
&self.columns[25]
}
pub fn SecretRecipeBook(&self) -> &ColumnData {
&self.columns[26]
}
pub fn CollectableMetadata(&self) -> &ColumnData {
&self.columns[27]
}
pub fn PatchNumber(&self) -> &ColumnData {
&self.columns[28]
}
pub fn AmountResult(&self) -> &ColumnData {
&self.columns[29]
}
pub fn AmountIngredient(&self) -> [&ColumnData; 8] {
[&self.columns[30],&self.columns[31],&self.columns[32],&self.columns[33],&self.columns[34],&self.columns[35],&self.columns[36],&self.columns[37],]
}
pub fn MaterialQualityFactor(&self) -> &ColumnData {
&self.columns[38]
}
pub fn CollectableMetadataKey(&self) -> &ColumnData {
&self.columns[39]
}
pub fn IsSecondary(&self) -> &ColumnData {
&self.columns[40]
}
pub fn CanQuickSynth(&self) -> &ColumnData {
&self.columns[41]
}
pub fn CanHq(&self) -> &ColumnData {
&self.columns[42]
}
pub fn ExpRewarded(&self) -> &ColumnData {
&self.columns[43]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[44]
}
pub fn IsSpecializationRequired(&self) -> &ColumnData {
&self.columns[45]
}
pub fn IsExpert(&self) -> &ColumnData {
&self.columns[46]
}
}
