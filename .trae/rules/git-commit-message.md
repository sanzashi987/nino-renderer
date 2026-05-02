---
scene: git_message
---
# Git Commit Message Rule

## 基本格式

- 生成的 git commit message 不超过 100 个字符。
- 必须遵循 `<type>(<scope>): <description>` 的形式。
- `type`、`scope`、`description` 都必须有实际含义。

## type 选择

- `feat`: 新增功能或能力。
- `fix`: 修复 bug 或错误行为。
- `refactor`: 不改变外部行为的重构。
- `chore`: 工程、脚本、规则、依赖等维护性改动。
- `docs`: 添加或完善文档。
- `test`: 添加或完善测试用例。

## scope 规范

- `scope` 应简洁，优先使用模块名、应用名、目录名或规则域。
- 推荐使用小写英文，如 `rules`、`calculation`、`frontend`、`system-data`。

## description 写法

- `description` 使用英文短语，尽量以动词开头。
- 只描述本次改动的核心目标，不写流水账。
- 避免 `update`、`fix bug`、`some changes` 这类低信息量描述。

## 长度限制

- commit message 的 description 长度必须控制在 100 个字符以内。

## 生成原则

- 根据本次改动的主要目的生成一条 commit message。
- 多个文件服务同一目标时，合并描述主目标即可。

## 本仓库建议

- 修改 `.trae/rules/` 下的规则文件时，优先考虑：
  - `chore(rules): ...`
  - `docs(rules): ...`
## 推荐示例

```text
docs(rules): refine git commit message guide
chore(rules): link calculation rule references
fix(calculation): correct flow editor route export
refactor(calculation): simplify strategy config handling
docs(rules): add docs type to commit message guide
```
