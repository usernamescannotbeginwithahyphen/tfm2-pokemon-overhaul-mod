# Move Category Icons

Move icons are generated from the official HOME move-category symbols listed by Bulbagarden Archives:

- Physical icon HOME.png
- Special icon HOME.png
- Status icon HOME.png

Source page:

```text
https://archives.bulbagarden.net/wiki/Category:Move_category_icons
```

The generator creates one icon for each Pokemon type and move category:

```text
mod/pokemon_moba/icons/move_categories/electric_physical.png
mod/pokemon_moba/icons/move_categories/electric_special.png
mod/pokemon_moba/icons/move_categories/electric_status.png
```

It also creates a shared sprite sheet:

```text
mod/pokemon_moba/icons/move_categories/move_categories#sheet.png
mod/pokemon_moba/icons/move_categories/move_categories#data.sprite_sheet
```

Run:

```powershell
.\.venv\Scripts\python.exe .\tools\generate-move-category-icons.py
```

