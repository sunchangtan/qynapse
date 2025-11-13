# 行业命名对照文档

本文档列出了量化交易系统中常用的行业标准缩写及其含义。

## 术语对照表

| 简写 | 全称 | 含义 | 常见于 |
|------|------|------|--------|
| Px | Price | 价格 | 撮合系统、行情结构 |
| Qty | Quantity | 数量 / 成交量 | 订单、成交 |
| Wgt | Weight | 权重（组合 / 仓位） | 策略信号 |
| Inst | Instrument | 标的（股票/合约） | 交易与行情数据 |
| Ts | Timestamp | 时间戳 | Tick/Bar/Clock |
| Ven / Venue | Venue | 交易所 / 市场 | 执行与行情 |
| Cur / Ccy | Currency | 币种 | 多币种系统 |
| OB | OrderBook | 盘口快照 | 市场微结构 |
| PnL | Profit and Loss | 盈亏 | Portfolio |
| O/H/L/C | Open / High / Low / Close | K线结构 | Bar 聚合 |
| Mid | Mid Price | 中间价 | 做市模型 |
| Slp | Slippage | 滑点 | 执行层 |
| Lat | Latency | 延迟 | 风控/监控 |
| Fee | Fee | 手续费 | 执行/账本 |

## 使用说明

这些缩写在代码、配置文件和日志中广泛使用，以提高可读性并保持与行业标准的一致性。在编写代码时，建议遵循这些命名约定。

### 示例

```rust
// 使用标准缩写的结构体定义
struct OrderData {
    inst: String,      // Instrument
    px: f64,           // Price
    qty: f64,          // Quantity
    ts: i64,           // Timestamp
}

struct BarData {
    o: f64,            // Open
    h: f64,            // High
    l: f64,            // Low
    c: f64,            // Close
}
```

## 注意事项

- 在公共API和文档中，优先使用完整名称以确保清晰度
- 在内部实现和性能关键代码中，可以使用缩写以提高简洁性
- 确保团队成员熟悉这些缩写以避免误解
