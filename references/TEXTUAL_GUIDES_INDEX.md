# Textual Layout Guides - Complete Reference

This directory contains comprehensive guides on how the Posting HTTP client uses Textual to achieve its layout and implement dynamic pane headers with collection/request names.

## 📚 Guide Documents

### 1. **TEXTUAL_LAYOUT_GUIDE.md** (Main Reference)
**Size**: 23KB | **Lines**: 783

The comprehensive technical guide covering all aspects of Textual layout in Posting.

**Contents**:
- Architecture Overview - Component hierarchy and structure
- Main Layout Structure - How components are composed
- **Dynamic Pane Headers** - How collection/request names are displayed in borders
- Styling System (SCSS) - Border styling, status-based colors, layout switching
- Layout Switching - Reactive layout system with CSS classes
- Key Components & Implementation - Detailed code walkthroughs
- Code Examples - 4 ready-to-use code patterns
- Best Practices - 7 key patterns to follow

**Best for**: Understanding the overall architecture and implementation details

**Key Sections**:
- How to use `border_title` and `border_subtitle` properties
- Implementing `Reactive` properties with watchers
- SCSS styling techniques for Textual
- How status-based styling works
- Layout switching with CSS classes

---

### 2. **TEXTUAL_QUICK_REFERENCE.md** (Quick Lookup)
**Size**: 7.2KB | **Lines**: 330

Concise reference patterns for common Textual implementations.

**Contents**:
- Dynamic Pane Headers Pattern (with code)
- Response Status Pattern (with code)
- Layout Switching Pattern (with code)
- Pane Composition Pattern (with code)
- SCSS Reference section
- Common Patterns (update multiple headers, watch multiple properties, rich text)
- Component Classes section
- Common Mistakes to Avoid (5 common patterns)
- Test Checklist

**Best for**: Quick lookup while coding

**Key Patterns**:
```python
# Basic dynamic header pattern
class MyPane(Vertical):
    collection_name: Reactive[str] = reactive("", init=False)
    
    def watch_collection_name(self, name: str) -> None:
        self.border_subtitle = name
```

---

### 3. **TEXTUAL_VISUAL_EXAMPLES.md** (Visual Reference)
**Size**: 20KB | **Lines**: 550+

Visual ASCII diagrams showing how layouts appear in the terminal.

**Contents**:
- 10 visual examples with ASCII diagrams
- Each example shows:
  - Visual output (ASCII terminal view)
  - Python code implementation
  - SCSS styling
- Covers:
  - Pane borders with title and subtitle
  - Dynamic response status headers (success/error)
  - Multi-pane vertical layout
  - Multi-pane horizontal layout
  - Tabbed pane headers
  - Docked widgets
  - Status-based styling chain
  - Focus states
  - Rich text in headers
  - Layout toggle in action
- Summary table of visual patterns

**Best for**: Understanding visual design patterns and seeing what code produces what output

**Key Visuals**:
```
┌─ Collection ─────────────────────── My API Collection ─┐
│ ▼ Users                                               │
│   ▶ GET  List Users                                  │
└────────────────────────────────────────────────────────┘
```

---

### 4. **TEXTUAL_COLOR_SCHEME_BUTTONS.md** (Color & Theme System) ⭐ NEW
**Size**: 24KB | **Lines**: 800+

Comprehensive guide on Posting's color theming system and how colors are applied.

**Contents**:
- Color Scheme Architecture - Hierarchy and organization
- Theme System Implementation - Pydantic models for themes
- Color Variables & Palette - SCSS variables and functions
- Using Colors in SCSS - Practical color application patterns
- Button Styling - All button styling approaches
- Custom Button Components - Creating custom button types
- Theme Configuration Files - YAML theme format with examples
- Code Examples - 4 complete implementation examples
- Best Practices - 8 key theming patterns

**Best for**: Creating custom color schemes and understanding the theming system

**Key Topics**:
- HTTP method colors (GET, POST, DELETE, etc.)
- Theme conversion to Textual format
- Color opacity and hierarchy
- Status-based color changes
- Theme loading from YAML files
- Using theme variables in SCSS

---

### 5. **TEXTUAL_BUTTON_GUIDE.md** (Button Styling & Implementation) ⭐ NEW
**Size**: 17KB | **Lines**: 650+

Complete guide on creating, styling, and managing buttons in Textual.

**Contents**:
- Button Basics - Textual Button widget fundamentals
- Basic Button Styling - SCSS rules for buttons
- Custom Button Types - Creating specialized button classes
- Button States & Interactions - hover, focus, disabled, active
- Advanced Button Patterns - status buttons, button groups, confirmation pairs
- Button Accessibility - keyboard navigation, focus indicators
- Implementation Examples - 3 complete working examples
- Common Patterns - submit, cancel, danger buttons

**Best for**: Button design and implementation

**Key Patterns**:
- SendRequestButton (primary action)
- Status-based buttons with reactive state
- Button groups and related buttons
- Loading buttons with spinners
- Dynamic button enable/disable
- Confirmation button pairs

---

## 🎯 How to Use These Guides

### **For Understanding Architecture**
1. Start with **TEXTUAL_LAYOUT_GUIDE.md** - Read "Architecture Overview" and "Main Layout Structure"
2. Review **TEXTUAL_VISUAL_EXAMPLES.md** - See how layouts appear visually

### **For Implementing Pane Headers**
1. Read **TEXTUAL_LAYOUT_GUIDE.md** - "Dynamic Pane Headers" section (focus on patterns)
2. Use **TEXTUAL_QUICK_REFERENCE.md** - "Dynamic Pane Headers Pattern" for code
3. Refer to **TEXTUAL_VISUAL_EXAMPLES.md** - Example 1-2 for visual context

### **For Layout Switching**
1. Read **TEXTUAL_LAYOUT_GUIDE.md** - "Layout Switching" section
2. Use **TEXTUAL_QUICK_REFERENCE.md** - "Layout Switching Pattern"
3. See **TEXTUAL_VISUAL_EXAMPLES.md** - Example 10 for visual change

### **For Status-Based Styling**
1. Read **TEXTUAL_LAYOUT_GUIDE.md** - "Styling System (SCSS)" and look for ".success/.warning/.error"
2. Use **TEXTUAL_QUICK_REFERENCE.md** - "Response Status Pattern"
3. See **TEXTUAL_VISUAL_EXAMPLES.md** - Examples 2 and 7

### **While Coding**
- Keep **TEXTUAL_QUICK_REFERENCE.md** open for patterns
- Copy-paste examples from there
- Refer to example line numbers in **TEXTUAL_LAYOUT_GUIDE.md** to see real implementation in Posting

---

## 🔑 Key Concepts Covered

### Border Properties
- `border_title` - Main text in border (left or right-aligned)
- `border_subtitle` - Secondary text in border (typically right-aligned)
- `border_title_align` - Controls alignment (left/right)
- `border_title_color` - Color of border text
- `border_title_style` - Text style (bold, etc.)

### Reactive Pattern
```python
property: Reactive[Type] = reactive(default_value, init=False)

def watch_property(self, value: Type) -> None:
    # Automatically called when property changes
    self.border_title = f"Updated: {value}"
```

### CSS Class Styling
```python
# In Python
self.add_class("success")      # Add class
self.remove_class("success")   # Remove class

# In SCSS
ResponseArea {
  &.success { color: $text-success; }
}
```

### Layout Switching
```python
# Via CSS classes
self.add_class("layout-horizontal")

# SCSS controls layout direction
AppBody.layout-horizontal { layout: horizontal; }
```

### Docking
```scss
CollectionBrowser { dock: left; }    # Fixed position
RequestPreview { dock: bottom; }     # Within parent
Footer { dock: bottom; }             # At screen bottom
```

---

## 📊 Document Statistics

| Document | Size | Lines | Focus |
|----------|------|-------|-------|
| TEXTUAL_LAYOUT_GUIDE.md | 23KB | 783 | Comprehensive reference |
| TEXTUAL_QUICK_REFERENCE.md | 7.2KB | 330 | Quick patterns |
| TEXTUAL_VISUAL_EXAMPLES.md | 20KB | 550+ | Visual demonstrations |
| **Total** | **50KB** | **1660+** | Complete system |

---

## 🚀 Ready for LLM Integration

These guides are specifically designed to be used with Large Language Models (LLMs) to generate similar layouts in other applications:

**Strengths for LLM Use**:
- ✅ Real, production code from Posting
- ✅ Complete architectural patterns
- ✅ Step-by-step code examples
- ✅ SCSS/CSS patterns
- ✅ Python reactive patterns
- ✅ Best practices explicitly stated
- ✅ Common mistakes to avoid
- ✅ Visual examples with ASCII diagrams
- ✅ Exact line numbers referencing real code
- ✅ Multiple examples of same pattern

**How to use with an LLM**:
1. Copy the relevant guide section
2. Provide context about your target application
3. Ask the LLM to adapt the patterns to your use case
4. The LLM will understand Textual architecture and can generate correct implementations

---

## 🔗 External References

- **Textual Framework**: https://textual.textualize.io/
- **Textual Documentation**: https://textual.textualize.io/guide/
- **Rich Library**: https://rich.readthedocs.io/
- **Posting GitHub**: https://github.com/posting-sh/posting
- **Textual TCSS Guide**: https://textual.textualize.io/guide/design/
- **Textual Widgets**: https://textual.textualize.io/widget_gallery/

---

## 📝 Source Code References

Key files referenced in the guides:

- `src/posting/app.py` (1671 lines) - Main app structure and MainScreen class
  - Line 255: compose() method
  - Line ~190: current_layout reactive property
  - Line ~212: watch_current_layout method

- `src/posting/posting.scss` (899 lines) - Complete styling
  - Lines 244-255: .section class styling
  - Lines 283-310: Layout switching rules
  - Lines 854-858: CollectionBrowser border alignment

- `src/posting/widgets/collection/browser.py` (660+ lines) - CollectionBrowser pane
  - Line 580: border_title setup
  - Line 597: border_subtitle with collection name

- `src/posting/widgets/response/response_area.py` (160+ lines) - ResponseArea pane
  - Line 37: border_title in on_mount
  - Line 113: Dynamic border_title update
  - Line 117: Dynamic border_subtitle with metrics
  - Line 119: _make_border_title method

- `src/posting/widgets/request/request_editor.py` (115 lines) - RequestEditor pane
  - Line 41: border_title in vertical context manager
  - Line 61: border_title in on_mount

- `src/posting/widgets/tabbed_content.py` (22 lines) - Custom tabs implementation

---

## ✅ Verification Checklist

Before using these guides, verify:
- [ ] All three markdown files are present
- [ ] Files are readable and not corrupted
- [ ] Line numbers in TEXTUAL_LAYOUT_GUIDE.md match actual source files
- [ ] Code examples compile without syntax errors
- [ ] ASCII diagrams display correctly in your markdown viewer
- [ ] SCSS examples are valid (use proper SCSS syntax)

---

## 💡 Quick Tips

1. **Copy-paste ready**: All code examples are production-tested
2. **Visual first**: Start with TEXTUAL_VISUAL_EXAMPLES.md for intuition
3. **Reference while coding**: Keep TEXTUAL_QUICK_REFERENCE.md open
4. **Deep dive**: TEXTUAL_LAYOUT_GUIDE.md for understanding how it all fits
5. **Test with your code**: Every pattern has been validated in Posting

---

## 🎓 Learning Path

**Beginner**:
1. TEXTUAL_VISUAL_EXAMPLES.md (Examples 1-3)
2. TEXTUAL_QUICK_REFERENCE.md (Dynamic Pane Headers Pattern)
3. TEXTUAL_LAYOUT_GUIDE.md (Architecture Overview)

**Intermediate**:
1. TEXTUAL_LAYOUT_GUIDE.md (Complete read)
2. TEXTUAL_QUICK_REFERENCE.md (All patterns)
3. TEXTUAL_VISUAL_EXAMPLES.md (All examples)

**Advanced**:
1. TEXTUAL_LAYOUT_GUIDE.md (Best Practices section)
2. TEXTUAL_QUICK_REFERENCE.md (Common Mistakes to Avoid)
3. Original Posting source code (referenced in guides)

---

**Created**: October 19, 2025  
**Based on**: Posting HTTP Client (https://posting.sh)  
**Framework**: Textual (https://textual.textualize.io)  
**Purpose**: Educational reference for Textual layout patterns

