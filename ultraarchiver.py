import os
import shutil
import subprocess
import threading
import sys
import tkinter as tk
from tkinter import ttk, filedialog, messagebox

APP_NAME = "UltraArchiver"

class UltraArchiver(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("UltraArchiver")
        self.geometry("820x610")
        self.minsize(720, 540)
        self.configure(bg="#10141d")
        self.files = []
        self.sevenzip = self.find_7zip()
        self._style()
        self._build()

    def find_7zip(self):
        app_dir = os.path.dirname(sys.executable) if getattr(sys, "frozen", False) else os.path.dirname(__file__)
        candidates = [
            shutil.which("7z"), shutil.which("7za"), shutil.which("7zz"),
            os.path.join(os.environ.get("ProgramFiles", r"C:\Program Files"), "7-Zip", "7z.exe"),
            os.path.join(os.environ.get("ProgramFiles(x86)", r"C:\Program Files (x86)"), "7-Zip", "7z.exe"),
            os.path.join(app_dir, "7z.exe"),
        ]
        return next((p for p in candidates if p and os.path.isfile(p)), None)

    def _style(self):
        s = ttk.Style(self)
        s.theme_use("clam")
        s.configure("TCombobox", fieldbackground="#202938", background="#202938", foreground="#f3f6fb", arrowcolor="#f3f6fb")
        s.configure("Horizontal.TProgressbar", troughcolor="#202938", background="#55d6a2", bordercolor="#202938", lightcolor="#55d6a2", darkcolor="#55d6a2")
        s.configure("Treeview", background="#171e2a", fieldbackground="#171e2a", foreground="#e8edf5", rowheight=29, borderwidth=0)
        s.configure("Treeview.Heading", background="#202938", foreground="#b9c7d9", relief="flat")
        s.map("Treeview", background=[("selected", "#294c5b")])

    def label(self, parent, text, size=10, color="#aebbd0", bold=False):
        return tk.Label(parent, text=text, bg=parent["bg"], fg=color, font=("Segoe UI", size, "bold" if bold else "normal"))

    def button(self, parent, text, command, primary=False):
        return tk.Button(parent, text=text, command=command, relief="flat", bg="#27b887" if primary else "#263244", fg="#071711" if primary else "#eaf0f8", activebackground="#42d19e" if primary else "#34445c", activeforeground="#ffffff", font=("Segoe UI", 10, "bold"), padx=15, pady=9, cursor="hand2", bd=0)

    def _build(self):
        header = tk.Frame(self, bg="#10141d")
        header.pack(fill="x", padx=26, pady=(22, 12))
        tk.Label(header, text="UltraArchiver", bg="#10141d", fg="#f3f7fc", font=("Segoe UI", 23, "bold")).pack(anchor="w")
        tk.Label(header, text="Архівування та розпакування файлів • Windows 10 / 11", bg="#10141d", fg="#8e9db2", font=("Segoe UI", 10)).pack(anchor="w", pady=(3, 0))
        main = tk.Frame(self, bg="#171e2a", highlightbackground="#283548", highlightthickness=1)
        main.pack(fill="both", expand=True, padx=26, pady=10)
        top = tk.Frame(main, bg="#171e2a")
        top.pack(fill="x", padx=18, pady=(16, 8))
        self.label(top, "ФАЙЛИ ДЛЯ АРХІВУ", 9, "#55d6a2", True).pack(side="left")
        self.button(top, "+ Додати файли", self.add_files).pack(side="right", padx=(8, 0))
        self.button(top, "＋ Додати папку", self.add_folder).pack(side="right")
        tree_frame = tk.Frame(main, bg="#171e2a")
        tree_frame.pack(fill="both", expand=True, padx=18, pady=(0, 10))
        self.tree = ttk.Treeview(tree_frame, columns=("path",), show="tree headings")
        self.tree.heading("#0", text="Назва")
        self.tree.heading("path", text="Розташування")
        self.tree.column("#0", width=220)
        self.tree.column("path", width=450)
        self.tree.pack(side="left", fill="both", expand=True)
        sb = ttk.Scrollbar(tree_frame, orient="vertical", command=self.tree.yview)
        sb.pack(side="right", fill="y")
        self.tree.configure(yscrollcommand=sb.set)
        row = tk.Frame(main, bg="#171e2a")
        row.pack(fill="x", padx=18, pady=7)
        self.label(row, "Формат архіву").pack(side="left")
        self.format_var = tk.StringVar(value="7z")
        ttk.Combobox(row, textvariable=self.format_var, values=["7z", "zip", "tar", "wim", "xz", "gzip", "bzip2"], state="readonly", width=12).pack(side="left", padx=(10, 26))
        self.label(row, "Рівень стиснення").pack(side="left")
        self.level_var = tk.StringVar(value="9 — Максимальний")
        ttk.Combobox(row, textvariable=self.level_var, values=["0 — Без стиснення", "1 — Швидкий", "3 — Швидкий+", "5 — Збалансований", "7 — Сильний", "9 — Максимальний"], state="readonly", width=21).pack(side="left", padx=10)
        out = tk.Frame(main, bg="#171e2a")
        out.pack(fill="x", padx=18, pady=9)
        self.label(out, "Зберегти як").pack(anchor="w")
        pathrow = tk.Frame(out, bg="#171e2a")
        pathrow.pack(fill="x", pady=(5, 0))
        self.output_var = tk.StringVar(value=os.path.join(os.path.expanduser("~"), "Desktop", "Archive.7z"))
        tk.Entry(pathrow, textvariable=self.output_var, bg="#202938", fg="#f3f6fb", insertbackground="white", relief="flat", font=("Segoe UI", 10)).pack(side="left", fill="x", expand=True, ipady=9)
        self.button(pathrow, "Огляд", self.choose_output).pack(side="left", padx=(8, 0))
        actions = tk.Frame(main, bg="#171e2a")
        actions.pack(fill="x", padx=18, pady=(9, 17))
        self.button(actions, "Створити архів", self.create_archive, True).pack(side="left")
        self.button(actions, "Розпакувати архів", self.extract_archive).pack(side="left", padx=9)
        self.button(actions, "Очистити список", self.clear).pack(side="right")
        footer = tk.Frame(self, bg="#10141d")
        footer.pack(fill="x", padx=26, pady=(0, 18))
        self.progress = ttk.Progressbar(footer, mode="indeterminate")
        self.progress.pack(fill="x", pady=(0, 7))
        self.status = tk.StringVar(value=self.status_text())
        tk.Label(footer, textvariable=self.status, bg="#10141d", fg="#91a2b8", font=("Segoe UI", 9)).pack(anchor="w")

    def status_text(self):
        return "7-Zip знайдено • Готово до роботи" if self.sevenzip else "7-Zip не знайдено • Встановіть 7-Zip"

    def add_paths(self, paths):
        for p in paths:
            p = os.path.abspath(p)
            if os.path.exists(p) and p not in self.files:
                self.files.append(p)
                self.tree.insert("", "end", text=os.path.basename(p) or p, values=(p,))

    def add_files(self):
        self.add_paths(filedialog.askopenfilenames(title="Виберіть файли"))

    def add_folder(self):
        p = filedialog.askdirectory(title="Виберіть папку")
        if p:
            self.add_paths([p])

    def choose_output(self):
        ext = self.format_var.get()
        p = filedialog.asksaveasfilename(defaultextension="." + ext, filetypes=[(ext.upper() + " archive", "*." + ext), ("Усі файли", "*.*")], initialfile="Archive." + ext)
        if p:
            self.output_var.set(p)

    def clear(self):
        self.files.clear()
        for item in self.tree.get_children():
            self.tree.delete(item)

    def ensure_7zip(self):
        self.sevenzip = self.find_7zip()
        if not self.sevenzip:
            messagebox.showerror("Потрібен 7-Zip", "Не знайдено 7z.exe. Встановіть 7-Zip або покладіть 7z.exe поруч із програмою.")
            return False
        return True

    def run_task(self, args, success_text):
        def worker():
            self.after(0, self.progress.start)
            try:
                proc = subprocess.run(args, capture_output=True, text=True, errors="replace", creationflags=getattr(subprocess, "CREATE_NO_WINDOW", 0))
                if proc.returncode in (0, 1):
                    msg = success_text if proc.returncode == 0 else success_text + "\nЗавершено з попередженнями."
                    self.after(0, lambda: messagebox.showinfo("UltraArchiver", msg))
                else:
                    err = (proc.stderr or proc.stdout or "Невідома помилка")[-2500:]
                    self.after(0, lambda e=err: messagebox.showerror("Помилка архіватора", e))
            except Exception as e:
                self.after(0, lambda: messagebox.showerror("Помилка", str(e)))
            finally:
                self.after(0, self.progress.stop)
        threading.Thread(target=worker, daemon=True).start()

    def create_archive(self):
        if not self.ensure_7zip():
            return
        if not self.files:
            messagebox.showwarning("Немає файлів", "Спочатку додайте файли або папки.")
            return
        out = self.output_var.get().strip()
        if not out:
            messagebox.showwarning("Шлях не задано", "Вкажіть ім'я та місце збереження архіву.")
            return
        fmt = self.format_var.get()
        try:
            level = int(self.level_var.get().split()[0])
        except Exception:
            level = 9
        os.makedirs(os.path.dirname(os.path.abspath(out)), exist_ok=True)
        self.run_task([self.sevenzip, "a", "-t" + fmt, "-mx=" + str(level), out] + self.files, "Архів успішно створено:\n" + out)

    def extract_archive(self):
        if not self.ensure_7zip():
            return
        archive = filedialog.askopenfilename(title="Виберіть архів", filetypes=[("Архіви", "*.7z *.zip *.rar *.tar *.tgz *.gz *.bz2 *.xz *.wim *.iso *.cab *.arj *.z *.lzma *.lzma2"), ("Усі файли", "*.*")])
        if not archive:
            return
        dest = filedialog.askdirectory(title="Куди розпакувати?")
        if not dest:
            return
        os.makedirs(dest, exist_ok=True)
        self.run_task([self.sevenzip, "x", archive, "-o" + dest, "-y"], "Розпакування завершено:\n" + dest)

if __name__ == "__main__":
    UltraArchiver().mainloop()
