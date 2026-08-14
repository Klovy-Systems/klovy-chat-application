use std::sync::atomic::{AtomicI64, Ordering};
use tauri::webview::{PageLoadEvent, Webview};
use tauri::{AppHandle, Manager, Runtime, UserAttentionType, WebviewWindow};

static LAST_COUNT: AtomicI64 = AtomicI64::new(0);

const INJECT: &str = r#"
(function () {
  if (window.__klovyBadgeWatch) return;
  window.__klovyBadgeWatch = true;
  var last = -1;
  function parseCount(title) {
    var m = String(title || "").match(/^\((\d+)\)/);
    return m ? parseInt(m[1], 10) : 0;
  }
  function apply() {
    var n = parseCount(document.title);
    if (n === last) return;
    last = n;
    try {
      var internals = window.__TAURI_INTERNALS__;
      if (internals && typeof internals.invoke === "function") {
        internals.invoke("set_unread_badge", { count: n });
      }
    } catch (e) {}
  }
  var titleEl = document.querySelector("title");
  if (titleEl && typeof MutationObserver !== "undefined") {
    new MutationObserver(apply).observe(titleEl, {
      childList: true,
      characterData: true,
      subtree: true
    });
  }
  setInterval(apply, 2000);
  apply();
})();
"#;

pub fn on_page_load<R: Runtime>(webview: &Webview<R>, event: PageLoadEvent) {
    if !matches!(event, PageLoadEvent::Finished) {
        return;
    }
    let _ = webview.eval(INJECT);
}

#[tauri::command]
pub fn set_unread_badge(app: AppHandle, count: i64) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "main window missing".to_string())?;
    apply_badge(&window, count);
    Ok(())
}

fn apply_badge(window: &WebviewWindow, count: i64) {
    let n = count.max(0);
    let prev = LAST_COUNT.swap(n, Ordering::SeqCst);
    if n == prev {
        return;
    }

    let _ = window.set_badge_count(if n > 0 { Some(n) } else { None });

    #[cfg(windows)]
    {
        let icon = if n > 0 {
            Some(windows_overlay_icon(n))
        } else {
            None
        };
        let _ = window.set_overlay_icon(icon);
    }

    if n > prev && !window.is_focused().unwrap_or(true) {
        let _ = window.request_user_attention(Some(UserAttentionType::Informational));
    }
}

/// Windows taskbar overlay: red circle with a white count (1–99).
#[cfg(windows)]
fn windows_overlay_icon(count: i64) -> tauri::image::Image<'static> {
    const SIZE: u32 = 16;
    let mut rgba = vec![0u8; (SIZE * SIZE * 4) as usize];
    let cx = (SIZE as f32 - 1.0) / 2.0;
    let r = SIZE as f32 / 2.0 - 0.35;
    for y in 0..SIZE {
        for x in 0..SIZE {
            let dx = x as f32 - cx;
            let dy = y as f32 - cx;
            let d = (dx * dx + dy * dy).sqrt();
            let i = ((y * SIZE + x) * 4) as usize;
            if d <= r - 0.45 {
                rgba[i] = 0xED;
                rgba[i + 1] = 0x42;
                rgba[i + 2] = 0x45;
                rgba[i + 3] = 255;
            } else if d < r + 0.45 {
                let a = ((r + 0.45 - d) * (255.0 / 0.9)).clamp(0.0, 255.0) as u8;
                rgba[i] = 0xED;
                rgba[i + 1] = 0x42;
                rgba[i + 2] = 0x45;
                rgba[i + 3] = a;
            }
        }
    }

    let shown = count.clamp(1, 99) as u32;
    if shown < 10 {
        blit_digit(&mut rgba, SIZE, shown, 5, 4, 2);
    } else {
        blit_digit(&mut rgba, SIZE, shown / 10, 2, 5, 1);
        blit_digit(&mut rgba, SIZE, shown % 10, 8, 5, 1);
    }

    tauri::image::Image::new_owned(rgba, SIZE, SIZE)
}

/// 3×5 pixel font, bits in the low 3 bits of each row.
#[cfg(windows)]
const FONT_3X5: [[u8; 5]; 10] = [
    [0b111, 0b101, 0b101, 0b101, 0b111], // 0
    [0b010, 0b110, 0b010, 0b010, 0b111], // 1
    [0b111, 0b001, 0b111, 0b100, 0b111], // 2
    [0b111, 0b001, 0b111, 0b001, 0b111], // 3
    [0b101, 0b101, 0b111, 0b001, 0b001], // 4
    [0b111, 0b100, 0b111, 0b001, 0b111], // 5
    [0b111, 0b100, 0b111, 0b101, 0b111], // 6
    [0b111, 0b001, 0b010, 0b010, 0b010], // 7
    [0b111, 0b101, 0b111, 0b101, 0b111], // 8
    [0b111, 0b101, 0b111, 0b001, 0b111], // 9
];

#[cfg(windows)]
fn blit_digit(rgba: &mut [u8], size: u32, digit: u32, ox: i32, oy: i32, scale: i32) {
    let glyph = FONT_3X5[(digit % 10) as usize];
    for row in 0..5 {
        for col in 0..3 {
            if glyph[row] & (1 << (2 - col)) == 0 {
                continue;
            }
            for sy in 0..scale {
                for sx in 0..scale {
                    let x = ox + col as i32 * scale + sx;
                    let y = oy + row as i32 * scale + sy;
                    if x < 0 || y < 0 || x >= size as i32 || y >= size as i32 {
                        continue;
                    }
                    let i = ((y as u32 * size + x as u32) * 4) as usize;
                    rgba[i] = 255;
                    rgba[i + 1] = 255;
                    rgba[i + 2] = 255;
                    rgba[i + 3] = 255;
                }
            }
        }
    }
}
