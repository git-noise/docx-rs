import * as wasm from "./pkg";

import { WidthType } from ".";
import { TableRow } from "./table-row";

import { TablePositionProperty } from "./json/bindings/TablePositionProperty";

export { TablePositionProperty } from "./json/bindings/TablePositionProperty";

export type TableAlignmentType = "center" | "left" | "right";
export type TableLayoutType = "fixed" | "autofit";

export class TablePosition {
  property: TablePositionProperty = {};

  leftFromText(n: number) {
    this.property.leftFromText = n;
    return this;
  }

  rightFromText(n: number) {
    this.property.rightFromText = n;
    return this;
  }

  verticalAnchor(n: string) {
    this.property.verticalAnchor = n;
    return this;
  }

  horizontalAnchor(n: string) {
    this.property.horizontalAnchor = n;
    return this;
  }

  positionXAlignment(n: string) {
    this.property.positionXAlignment = n;
    return this;
  }

  positionYAlignment(n: string) {
    this.property.positionYAlignment = n;
    return this;
  }

  positionX(n: number) {
    this.property.positionX = n;
    return this;
  }

  positionY(n: number) {
    this.property.positionY = n;
    return this;
  }

  build() {
    let p = wasm.createTablePosition();
    if (this.property.leftFromText != null) {
      p = p.left_from_text(this.property.leftFromText);
    }
    if (this.property.rightFromText != null) {
      p = p.left_from_text(this.property.rightFromText);
    }
    if (this.property.verticalAnchor != null) {
      p = p.vertical_anchor(this.property.verticalAnchor);
    }
    if (this.property.horizontalAnchor != null) {
      p = p.horizontal_anchor(this.property.horizontalAnchor);
    }
    if (this.property.positionXAlignment != null) {
      p = p.position_x_alignment(this.property.positionXAlignment);
    }
    if (this.property.positionYAlignment != null) {
      p = p.position_y_alignment(this.property.positionYAlignment);
    }
    if (this.property.positionX != null) {
      p = p.position_x(this.property.positionX);
    }
    if (this.property.positionY != null) {
      p = p.position_y(this.property.positionY);
    }
    return p;
  }
}

export type TableProperty = {
  indent?: number;
  align?: TableAlignmentType;
  width?: number;
  styleId?: string;
  cellMargins: {
    top: { val: number; type: WidthType };
    left: { val: number; type: WidthType };
    bottom: { val: number; type: WidthType };
    right: { val: number; type: WidthType };
  };
  layout?: TableLayoutType;
  position?: TablePosition;
};

export const createDefaultTableCellMargins = () => {
  return {
    top: { val: 0, type: "dxa" },
    left: { val: 55, type: "dxa" },
    bottom: { val: 0, type: "dxa" },
    right: { val: 55, type: "dxa" },
  } as const;
};

export class Table {
  hasNumberings = false;
  rows: TableRow[] = [];
  grid: number[] = [];
  property: TableProperty = {
    cellMargins: createDefaultTableCellMargins(),
  };

  addRow(row: TableRow) {
    if (row.hasNumberings) {
      this.hasNumberings = true;
    }
    this.rows.push(row);
    return this;
  }

  style(id: string) {
    this.property.styleId = id;
    return this;
  }

  setGrid(grid: number[]) {
    this.grid = grid;
    return this;
  }

  indent(v: number) {
    this.property.indent = v;
    return this;
  }

  align(v: TableAlignmentType) {
    this.property.align = v;
    return this;
  }

  layout(l: TableLayoutType) {
    this.property.layout = l;
    return this;
  }

  width(w: number) {
    this.property.width = w;
    return this;
  }

  cellMargins(top: number, right: number, bottom: number, left: number) {
    this.property.cellMargins = {
      top: { val: top, type: "dxa" },
      left: { val: left, type: "dxa" },
      bottom: { val: bottom, type: "dxa" },
      right: { val: right, type: "dxa" },
    };
    return this;
  }

  cellMarginTop(v: number, t: WidthType) {
    this.property.cellMargins.top = { val: v, type: t };
    return this;
  }

  cellMarginLeft(v: number, t: WidthType) {
    this.property.cellMargins.left = { val: v, type: t };
    return this;
  }

  cellMarginRight(v: number, t: WidthType) {
    this.property.cellMargins.right = { val: v, type: t };
    return this;
  }

  cellMarginBottom(v: number, t: WidthType) {
    this.property.cellMargins.bottom = { val: v, type: t };
    return this;
  }

  position(p: TablePosition) {
    this.property.position = p;
    return this;
  }

  build() {
    let table = wasm.createTable();
    this.rows.forEach((r) => {
      let row = wasm.createTableRow();
      r.cells.forEach((c) => {
        const cell = c.build();
        row = row.add_cell(cell);
      });

      if (r.height) {
        row = row.row_height(r.height);
      }

      if (r.del) {
        row = row.delete(r.del.author, r.del.date);
      }

      if (r.ins) {
        row = row.insert(r.ins.author, r.ins.date);
      }

      if (r.hRule) {
        switch (r.hRule) {
          case "auto": {
            row = row.height_rule(wasm.HeightRule.Auto);
            break;
          }
          case "atLeast": {
            row = row.height_rule(wasm.HeightRule.AtLeast);
            break;
          }
          case "exact": {
            row = row.height_rule(wasm.HeightRule.Exact);
            break;
          }
        }
      }
      table = table.add_row(row);
    });

    table = table.set_grid(new Uint32Array(this.grid));

    if (this.property.styleId) {
      table = table.style(this.property.styleId);
    }

    if (this.property.position) {
      table = table.position(this.property.position.build());
    }

    table = setTableProperty(table, this.property);

    return table;
  }
}

export const convertWidthType = (t: string) => {
  switch (t) {
    case "nil":
    case "Nil":
      return wasm.WidthType.Nil;
    case "Pct":
    case "pct":
      return wasm.WidthType.Pct;
    case "DXA":
    case "dxa":
      return wasm.WidthType.Dxa;
    case "Auto":
    case "auto":
      return wasm.WidthType.Auto;
    default:
      return wasm.WidthType.Dxa;
  }
};

export const setTableProperty = <T extends wasm.Table | wasm.Style>(
  target: T,
  property: TableProperty
): T => {
  if (target instanceof wasm.Table) {
    target = target.indent(property.indent ?? 0) as T;
  } else if (target instanceof wasm.Style) {
    target = target.table_indent(property.indent ?? 0) as T;
  }

  if (property.cellMargins) {
    const { top, right, bottom, left } = property.cellMargins;
    target = target
      .cell_margin_top(top.val, convertWidthType(top.type))
      .cell_margin_right(right.val, convertWidthType(right.type))
      .cell_margin_bottom(bottom.val, convertWidthType(bottom.type))
      .cell_margin_left(left.val, convertWidthType(left.type)) as T;
  }

  const align = ((): wasm.TableAlignmentType | null => {
    switch (property.align) {
      case "center": {
        return wasm.TableAlignmentType.Center;
      }
      case "right": {
        return wasm.TableAlignmentType.Right;
      }
      case "left": {
        return wasm.TableAlignmentType.Left;
      }
      default:
        return null;
    }
  })();

  if (align != null) {
    if (target instanceof wasm.Table) {
      target = target.align(align) as T;
    } else if (target instanceof wasm.Style) {
      target = target.table_align(align) as T;
    }
  }

  switch (property.layout) {
    case "fixed": {
      target = target.layout(wasm.TableLayoutType.Fixed) as T;
      break;
    }
    case "autofit": {
      target = target.layout(wasm.TableLayoutType.Autofit) as T;
      break;
    }
  }
  return target;
};
