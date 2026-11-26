# Parity ledger F0: UI foundation

Slice F0, written 25/11/2026. Decision D11 followed at its default: the TROQ stays look, as semantic tokens in `packages/theme/src/tokens.css`. Legacy root is `troq-front/troqzone-business/src` unless a column says otherwise. Package names are `@mungini/theme` and `@mungini/ui-web`.

## Section 4 primitives

| Primitive | Legacy location | Mungini location | Status |
|---|---|---|---|
| Button | `components/ui/button.tsx` | `ui-web/src/button/Button.tsx`, `buttonVariants.ts`, `useButton.ts` | done |
| Icon-only button | `components/stays/properties/StayRefreshButton.tsx` and icon buttons in stays footers | `ui-web/src/button/IconTooltipButton.tsx` | done |
| Input | `components/ui/input.tsx`, `ui/search-input.tsx` | `ui-web/src/form/Input.tsx` (leading icon and trailing slot replace SearchInput) | done |
| Textarea | `components/ui/textarea.tsx` | `ui-web/src/form/Textarea.tsx` | done |
| Select | `components/ui/select.tsx` | `ui-web/src/form/Select.tsx` (status dot on options, as in `StayPropertyFilters`) | done |
| SearchableSelect | none in legacy (Command plus Popover used ad hoc) | `ui-web/src/form/SearchableSelect.tsx`, `useSearchableSelect.ts` | done |
| DatePicker | `components/ui/date-picker.tsx`, `ui/hooks/useDatePicker.ts` (react-datepicker) | `ui-web/src/date/DatePicker.tsx`, own `Calendar.tsx` | done; visible month follows externally changed selection; runtime check open |
| DateRangePicker | `components/stays/dates/StayDateRangePicker.tsx`, `ui/calendar.tsx` (react-day-picker) | `ui-web/src/date/DateRangePicker.tsx`, `rangeSelection.ts` | done |
| DateTimePicker | none in legacy | `ui-web/src/date/DateTimePicker.tsx`, `useDateTimePicker.ts`, `format/zonedInstant.ts` | done |
| AmountInput | none in legacy (plain inputs) | `ui-web/src/form/AmountInput.tsx`, `amountFormat.ts` | done |
| PhoneField | `components/auth/PhoneField.tsx`, `auth/hooks/usePhoneField.ts`, `utils/auth/phoneCountries.ts` | `ui-web/src/phone/` | done; asynchronous detection keeps the fallback on failure and does not overwrite user input; shared dialing codes retain the selected country while mounted; runtime check open |
| Checkbox | `components/ui/checkbox.tsx` | `ui-web/src/form/Checkbox.tsx` | done |
| Switch | `components/ui/switch.tsx` | `ui-web/src/form/Switch.tsx` | done |
| SegmentedControl | `components/stays/StayTabs.tsx`, `stays/common/StayTabStrip.tsx` | `ui-web/src/layout/SegmentedControl.tsx` | done |
| UnderlineTabs | none in legacy (`ui/tabs.tsx` is the segmented style) | `ui-web/src/layout/UnderlineTabs.tsx` | done |
| StatusDot | `components/stays/properties/StayPropertyStatus.tsx` | `ui-web/src/display/StatusDot.tsx` | done |
| TonePill | none in legacy | `ui-web/src/display/TonePill.tsx` | done |
| Badge | `components/ui/badge.tsx` | `ui-web/src/display/Badge.tsx` | done |
| EmptyState | none in legacy (inline empty text) | `ui-web/src/display/EmptyState.tsx` | done |
| Skeleton | `components/ui/skeleton.tsx` | `ui-web/src/display/Skeleton.tsx` | done |
| Dialog | `components/ui/dialog.tsx` | `ui-web/src/overlay/Dialog.tsx`, `OverlayHeader.tsx` | done |
| Popover | `components/ui/popover.tsx` | `ui-web/src/overlay/Popover.tsx` | done |
| Command | `components/ui/command.tsx` | `ui-web/src/overlay/Command.tsx` | done |
| Drawer | `components/ui/sheet.tsx` (right side) | `ui-web/src/overlay/Drawer.tsx` | done |
| SlideUpDrawer | `components/ui/sheet.tsx` (bottom side) | `ui-web/src/overlay/SlideUpDrawer.tsx` | done |
| ActionMenu | `components/ui/dropdown-menu.tsx` (listed, not read in full) | `ui-web/src/menu/ActionMenu.tsx` | done |
| RowContextMenu | `components/ui/context-menu.tsx` (listed, not read in full) | `ui-web/src/menu/RowContextMenu.tsx` | done |
| ConfirmDialog | `components/common/ConfirmModal.tsx`, `ui/alert-dialog.tsx` | `ui-web/src/overlay/ConfirmDialog.tsx` | done |
| BulkActionBar | none in legacy | `ui-web/src/table/BulkActionBar.tsx` | done |
| PagedTable | `components/stays/reservations/StayReservationList.tsx`, `ui/table.tsx` | `ui-web/src/table/PagedTable.tsx`, `PagedTableRow.tsx`, `usePagedTable.ts` | done |
| Pagination | `components/stays/reservations/StayReservationsFooter.tsx` (the `common/Pagination.tsx` product-grid pager is not carried over) | `ui-web/src/table/Pagination.tsx`, `usePagination.ts` | done |
| SortableHeader | none in legacy | `ui-web/src/table/SortableHeader.tsx`, `sortState.ts` | done |
| Avatar | `components/ui/avatar.tsx` | `ui-web/src/display/Avatar.tsx` | done |
| FadeIn | none in legacy | `ui-web/src/display/FadeIn.tsx` | done |
| Panel | `components/stays/properties/StayPanel.tsx` (the refresh slot is dropped, compose an `IconTooltipButton` in the header) | `ui-web/src/layout/Panel.tsx` | done |
| Section | `components/stays/properties/StaySection.tsx` | `ui-web/src/layout/Section.tsx` | done |
| Toast | `components/ui/toast.tsx`, `toaster.tsx`, `hooks/use-toast.ts` | `ui-web/src/toast/` (store, viewport, item, no dependency, plain card) | done |
| RichTextEditor | `components/common/RichTextEditor.tsx`, `hooks/common/useCkEditor.ts` (CKEditor 5, full toolbar) | `ui-web/src/rich-text/` (CKEditor 5 GPL, reduced toolbar, subpath export `@mungini/ui-web/rich-text`) | done |
| useGridPrefs | none in legacy | `ui-web/src/table/useGridPrefs.ts` | done; stable across equivalent column arrays, merges new default widths with valid saved widths and respects non-hideable columns; runtime check open |

## Tokens, tones, formatters

| Item | Legacy location | Mungini location | Status |
|---|---|---|---|
| Colour, radius, shadow tokens, light and dark | `styles/troqzone-theme.css`, `styles/stays-shell.css`, `index.css` | `packages/theme/src/tokens.css`, `theme.css` | done |
| Text size tokens micro, label, body, title | `styles/troqzone-theme.css` | `packages/theme/src/theme.css` | done |
| Legacy extra size tokens (compact, ui, section, page, heading, display, brand, hero, and the redefined xs to 4xl) | `styles/troqzone-theme.css` | not carried over, the design file allows four sizes | dropped by decision D11 (stays look uses four tokens) |
| Text tones | `utils/stays/textTones.ts` | `packages/theme/src/tones.ts` (token based, no `dark:` classes) | done |
| Status dot tones | `DESIGN.md` status table | `packages/theme/src/tones.ts` | done |
| Day and timestamp format | `utils/stays/stayDateTime.ts` | `packages/utils/src/date/formatDay.ts`, re-exported by `ui-web` (time zone is an argument, no localStorage city) | done |
| Money formatter | `utils/stays/formatCdf.ts` | `packages/utils/src/money.ts`, re-exported by `ui-web` | done |
| Keyframes and reduced motion | `tailwindcss-animate` plugin, `stays-shell.css` | `packages/theme/src/theme.css` | done |
| Font | `Plus Jakarta Sans` in `index.css`, Onest in `stays-shell.css` | `--font-sans-stack` starts with Onest, font file loading is the app's job | done, apps load the font |
| Tailwind v4 wiring | `styles/tailwind.css` | `packages/theme/src/theme.css` (`@source` covers `ui-web`) | done |

## Shell and app pieces

| Item | Legacy location | Mungini location | Status |
|---|---|---|---|
| Stays layout shell | `components/layouts/StaysLayout.tsx`, `styles/stays-shell.css` shell rules | `apps/host` and `apps/admin` shells | open, P0-6 and lead step |
| Sidebar, active nav marker, top bar | `styles/business-sidebar.css`, `components/admin/nav` | app shells and module registry | open, P0-6 and lead step |
| Breadcrumb | `components/stays/StayBreadcrumb.tsx` | `packages/ui-web/src/shell/Breadcrumb.tsx` | open: shared presentation available; route-derived items and integration await real routes |
| Page header stack | `components/common/PageHeader.tsx`, `ShellPageHeader.tsx` | `packages/ui-web/src/shell/ShellHeader.tsx` in host and admin | open: top bar shared; route title and page actions await real routes |
| Theme toggle | `components/ui/ThemeToggle.tsx` | `packages/ui-web/src/theme/ThemeToggle.tsx`, host and admin home headers | done, rendered verification pending |
| Inline confirm panel | `components/stays/confirm/StayConfirmPanel.tsx` | screens that use it (ConfirmDialog covers the modal form) | open, F3 |
| Collapsible money breakdown | `components/stays/money/StayMoneyBreakdown.tsx` | feature component, not a section 4 primitive | open, F3 |

Theme toggle runtime: initialization now runs in the browser, tolerates unavailable storage, follows system changes until an explicit choice, and accepts cross-tab changes. Runtime verification remains open.
