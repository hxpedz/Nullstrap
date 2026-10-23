use windows_sys::Win32::Globalization::GetUserDefaultUILanguage;

struct Texts {
    ask: &'static str,
    failed: &'static str,
    unpack: &'static str,
}

const EN: Texts = Texts {
    ask: "Nullstrap needs the Microsoft .NET {0} Desktop Runtime to run. Install it now?\n\nWindows may ask for permission during the install.",
    failed: "The .NET {0} Desktop Runtime could not be installed automatically. The Microsoft download page will open now. Install it from there, then start Nullstrap again.",
    unpack: "Nullstrap could not be started. Make sure there is free disk space and that your antivirus is not blocking Nullstrap, then try again.",
};

const AR: Texts = Texts {
    ask: "يحتاج Nullstrap إلى Microsoft .NET {0} Desktop Runtime للعمل. هل تريد تثبيته الآن؟\n\nقد يطلب Windows الإذن أثناء التثبيت.",
    failed: "تعذر تثبيت .NET {0} Desktop Runtime تلقائيًا. ستُفتح صفحة التنزيل من Microsoft الآن. ثبّته من هناك، ثم شغّل Nullstrap مرة أخرى.",
    unpack: "تعذر تشغيل Nullstrap. تأكد من وجود مساحة فارغة على القرص ومن أن برنامج مكافحة الفيروسات لا يحظر Nullstrap، ثم حاول مرة أخرى.",
};

const BG: Texts = Texts {
    ask: "Nullstrap се нуждае от Microsoft .NET {0} Desktop Runtime, за да работи. Да го инсталирате ли сега?\n\nWindows може да поиска разрешение по време на инсталацията.",
    failed: "Не успяхме да инсталираме автоматично .NET {0} Desktop Runtime. Сега ще се отвори страницата за изтегляне на Microsoft. Инсталирайте го оттам и след това стартирайте Nullstrap отново.",
    unpack: "Nullstrap не можа да се стартира. Уверете се, че има свободно място на диска и че антивирусната Ви програма не блокира Nullstrap, след което опитайте отново.",
};

const CS: Texts = Texts {
    ask: "Nullstrap ke svému běhu potřebuje Microsoft .NET {0} Desktop Runtime. Chcete jej nyní nainstalovat?\n\nSystém Windows může během instalace požádat o oprávnění.",
    failed: "Nepodařilo se automaticky nainstalovat .NET {0} Desktop Runtime. Nyní se otevře stránka pro stažení od společnosti Microsoft. Nainstalujte jej odtud a poté znovu spusťte Nullstrap.",
    unpack: "Nullstrap se nepodařilo spustit. Ujistěte se, že je na disku volné místo a že váš antivirus Nullstrap neblokuje, a zkuste to znovu.",
};

const DE: Texts = Texts {
    ask: "Nullstrap benötigt die Microsoft .NET {0} Desktop Runtime. Möchtest du sie jetzt installieren?\n\nWindows fragt während der Installation eventuell nach einer Berechtigung.",
    failed: "Die .NET {0} Desktop Runtime konnte nicht automatisch installiert werden. Die Downloadseite von Microsoft wird jetzt geöffnet. Installiere sie dort und starte Nullstrap danach erneut.",
    unpack: "Nullstrap konnte nicht gestartet werden. Stelle sicher, dass genug Speicherplatz frei ist und dein Virenschutz Nullstrap nicht blockiert, und versuche es dann erneut.",
};

const ES: Texts = Texts {
    ask: "Nullstrap necesita Microsoft .NET {0} Desktop Runtime para funcionar. ¿Quieres instalarlo ahora?\n\nEs posible que Windows te pida permiso durante la instalación.",
    failed: "No se pudo instalar .NET {0} Desktop Runtime automáticamente. Ahora se abrirá la página de descarga de Microsoft. Instálalo desde allí y luego vuelve a abrir Nullstrap.",
    unpack: "No se pudo iniciar Nullstrap. Asegúrate de tener espacio libre en el disco y de que tu antivirus no esté bloqueando Nullstrap, y vuelve a intentarlo.",
};

const FA: Texts = Texts {
    ask: "Nullstrap برای اجرا به Microsoft .NET {0} Desktop Runtime نیاز دارد. آیا می‌خواهید آن را اکنون نصب کنید؟\n\nممکن است Windows هنگام نصب از شما اجازه بخواهد.",
    failed: "نصب خودکار .NET {0} Desktop Runtime ممکن نشد. صفحه دانلود Microsoft اکنون باز می‌شود. آن را از آنجا نصب کنید و سپس Nullstrap را دوباره اجرا کنید.",
    unpack: "Nullstrap اجرا نشد. مطمئن شوید فضای خالی کافی روی دیسک دارید و آنتی‌ویروس شما Nullstrap را مسدود نمی‌کند، سپس دوباره تلاش کنید.",
};

const FI: Texts = Texts {
    ask: "Nullstrap tarvitsee Microsoft .NET {0} Desktop Runtimen toimiakseen. Haluatko asentaa sen nyt?\n\nWindows saattaa pyytää lupaa asennuksen aikana.",
    failed: ".NET {0} Desktop Runtimea ei voitu asentaa automaattisesti. Microsoftin lataussivu avautuu nyt. Asenna se sieltä ja käynnistä Nullstrap sitten uudelleen.",
    unpack: "Nullstrapia ei voitu käynnistää. Varmista, että levyllä on vapaata tilaa eikä virustorjuntasi estä Nullstrapia, ja yritä sitten uudelleen.",
};

const FIL: Texts = Texts {
    ask: "Kailangan ng Nullstrap ang Microsoft .NET {0} Desktop Runtime para gumana. Gusto mo ba itong i-install ngayon?\n\nMaaaring humingi ng pahintulot ang Windows habang nag-i-install.",
    failed: "Hindi awtomatikong ma-install ang .NET {0} Desktop Runtime. Bubuksan na ngayon ang download page ng Microsoft. I-install mo ito mula roon, pagkatapos ay buksan muli ang Nullstrap.",
    unpack: "Hindi masimulan ang Nullstrap. Siguraduhing may libreng espasyo sa disk at hindi bina-block ng iyong antivirus ang Nullstrap, pagkatapos ay subukan muli.",
};

const FR: Texts = Texts {
    ask: "Nullstrap a besoin de Microsoft .NET {0} Desktop Runtime pour fonctionner. Voulez-vous l'installer maintenant ?\n\nWindows peut vous demander une autorisation pendant l'installation.",
    failed: "Impossible d'installer automatiquement .NET {0} Desktop Runtime. La page de téléchargement de Microsoft va s'ouvrir. Installez-le depuis cette page, puis relancez Nullstrap.",
    unpack: "Impossible de démarrer Nullstrap. Vérifiez qu'il reste de l'espace disque et que votre antivirus ne bloque pas Nullstrap, puis réessayez.",
};

const HR: Texts = Texts {
    ask: "Nullstrap za rad treba Microsoft .NET {0} Desktop Runtime. Želiš li ga sada instalirati?\n\nWindows bi mogao tražiti dopuštenje tijekom instalacije.",
    failed: ".NET {0} Desktop Runtime nije bilo moguće automatski instalirati. Sada će se otvoriti Microsoftova stranica za preuzimanje. Instaliraj ga odande, a zatim ponovno pokreni Nullstrap.",
    unpack: "Nullstrap se nije mogao pokrenuti. Provjeri imaš li slobodnog prostora na disku i ne blokira li tvoj antivirusni program Nullstrap, pa pokušaj ponovno.",
};

const HU: Texts = Texts {
    ask: "A Nullstrap futtatásához Microsoft .NET {0} Desktop Runtime szükséges. Szeretnéd most telepíteni?\n\nA Windows engedélyt kérhet a telepítés során.",
    failed: "A .NET {0} Desktop Runtime automatikus telepítése nem sikerült. Most megnyílik a Microsoft letöltési oldala. Telepítsd onnan, majd indítsd újra a Nullstrapet.",
    unpack: "A Nullstrap nem indult el. Győződj meg róla, hogy van szabad hely a lemezen, és a vírusirtód nem blokkolja a Nullstrapet, majd próbáld újra.",
};

const ID: Texts = Texts {
    ask: "Nullstrap memerlukan Microsoft .NET {0} Desktop Runtime untuk berjalan. Instal sekarang?\n\nWindows mungkin meminta izin selama instalasi.",
    failed: ".NET {0} Desktop Runtime tidak dapat diinstal secara otomatis. Halaman unduhan Microsoft akan dibuka sekarang. Instal dari sana, lalu jalankan Nullstrap lagi.",
    unpack: "Nullstrap tidak dapat dijalankan. Pastikan ruang disk Anda cukup dan antivirus Anda tidak memblokir Nullstrap, lalu coba lagi.",
};

const IT: Texts = Texts {
    ask: "Nullstrap ha bisogno di Microsoft .NET {0} Desktop Runtime per funzionare. Vuoi installarlo ora?\n\nWindows potrebbe chiederti l'autorizzazione durante l'installazione.",
    failed: "Non è stato possibile installare automaticamente .NET {0} Desktop Runtime. Ora si aprirà la pagina di download di Microsoft. Installalo da lì, poi avvia di nuovo Nullstrap.",
    unpack: "Impossibile avviare Nullstrap. Assicurati di avere spazio libero sul disco e che il tuo antivirus non blocchi Nullstrap, poi riprova.",
};

const JA: Texts = Texts {
    ask: "Nullstrap の実行には Microsoft .NET {0} Desktop Runtime が必要です。今すぐインストールしますか？\n\nインストール中に Windows が許可を求める場合があります。",
    failed: ".NET {0} Desktop Runtime を自動でインストールできませんでした。Microsoft のダウンロード ページを開きます。そこからインストールしてから、Nullstrap をもう一度起動してください。",
    unpack: "Nullstrap を起動できませんでした。ディスクに空き容量があり、ウイルス対策ソフトが Nullstrap をブロックしていないことを確認してから、もう一度お試しください。",
};

const KO: Texts = Texts {
    ask: "Nullstrap을 실행하려면 Microsoft .NET {0} Desktop Runtime이 필요합니다. 지금 설치하시겠습니까?\n\n설치 중에 Windows가 권한을 요청할 수 있습니다.",
    failed: ".NET {0} Desktop Runtime을 자동으로 설치하지 못했습니다. 지금 Microsoft 다운로드 페이지가 열립니다. 그곳에서 설치한 다음 Nullstrap을 다시 실행하십시오.",
    unpack: "Nullstrap을 시작할 수 없습니다. 디스크 공간이 충분한지, 바이러스 백신이 Nullstrap을 차단하지 않는지 확인한 후 다시 시도하십시오.",
};

const LT: Texts = Texts {
    ask: "Nullstrap veikimui reikia Microsoft .NET {0} Desktop Runtime. Ar norite jį įdiegti dabar?\n\nDiegimo metu Windows gali paprašyti leidimo.",
    failed: "Nepavyko automatiškai įdiegti .NET {0} Desktop Runtime. Dabar atsivers Microsoft atsisiuntimo puslapis. Įdiekite jį iš ten, tada vėl paleiskite Nullstrap.",
    unpack: "Nepavyko paleisti Nullstrap. Įsitikinkite, kad diske yra laisvos vietos ir jūsų antivirusinė programa neblokuoja Nullstrap, tada bandykite dar kartą.",
};

const MS: Texts = Texts {
    ask: "Nullstrap memerlukan Microsoft .NET {0} Desktop Runtime untuk berfungsi. Pasang sekarang?\n\nWindows mungkin meminta kebenaran semasa pemasangan.",
    failed: ".NET {0} Desktop Runtime tidak dapat dipasang secara automatik. Halaman muat turun Microsoft akan dibuka sekarang. Pasangkannya dari sana, kemudian buka Nullstrap semula.",
    unpack: "Nullstrap tidak dapat dimulakan. Pastikan cakera anda mempunyai ruang kosong dan antivirus anda tidak menyekat Nullstrap, kemudian cuba lagi.",
};

const NL: Texts = Texts {
    ask: "Nullstrap heeft Microsoft .NET {0} Desktop Runtime nodig om te werken. Wil je het nu installeren?\n\nWindows kan tijdens de installatie om toestemming vragen.",
    failed: ".NET {0} Desktop Runtime kon niet automatisch worden geïnstalleerd. De downloadpagina van Microsoft wordt nu geopend. Installeer het daar en start Nullstrap daarna opnieuw.",
    unpack: "Nullstrap kon niet worden gestart. Controleer of er vrije schijfruimte is en of je antivirus Nullstrap niet blokkeert, en probeer het opnieuw.",
};

const PL: Texts = Texts {
    ask: "Nullstrap do działania potrzebuje Microsoft .NET {0} Desktop Runtime. Czy chcesz go teraz zainstalować?\n\nPodczas instalacji system Windows może poprosić o zgodę.",
    failed: "Nie udało się automatycznie zainstalować .NET {0} Desktop Runtime. Teraz otworzy się strona pobierania Microsoft. Zainstaluj go stamtąd, a potem uruchom Nullstrap ponownie.",
    unpack: "Nie udało się uruchomić Nullstrap. Upewnij się, że masz wolne miejsce na dysku i że Twój program antywirusowy nie blokuje Nullstrap, a potem spróbuj ponownie.",
};

const PT_BR: Texts = Texts {
    ask: "O Nullstrap precisa do Microsoft .NET {0} Desktop Runtime para funcionar. Deseja instalá-lo agora?\n\nO Windows pode pedir permissão durante a instalação.",
    failed: "Não foi possível instalar o .NET {0} Desktop Runtime automaticamente. A página de download da Microsoft será aberta agora. Instale-o por lá e depois abra o Nullstrap novamente.",
    unpack: "Não foi possível iniciar o Nullstrap. Verifique se há espaço livre no disco e se o seu antivírus não está bloqueando o Nullstrap, e tente novamente.",
};

const RO: Texts = Texts {
    ask: "Nullstrap are nevoie de Microsoft .NET {0} Desktop Runtime pentru a funcționa. Vrei să îl instalezi acum?\n\nWindows ar putea cere permisiunea în timpul instalării.",
    failed: ".NET {0} Desktop Runtime nu a putut fi instalat automat. Se va deschide acum pagina de descărcare Microsoft. Instalează-l de acolo, apoi pornește din nou Nullstrap.",
    unpack: "Nullstrap nu a putut porni. Asigură-te că ai spațiu liber pe disc și că antivirusul tău nu blochează Nullstrap, apoi încearcă din nou.",
};

const RU: Texts = Texts {
    ask: "Для работы Nullstrap нужен Microsoft .NET {0} Desktop Runtime. Установить его сейчас?\n\nВо время установки Windows может запросить разрешение.",
    failed: "Не удалось автоматически установить .NET {0} Desktop Runtime. Сейчас откроется страница загрузки Microsoft. Установите его оттуда, а затем снова запустите Nullstrap.",
    unpack: "Не удалось запустить Nullstrap. Убедитесь, что на диске есть свободное место и Ваш антивирус не блокирует Nullstrap, затем повторите попытку.",
};

const SV: Texts = Texts {
    ask: "Nullstrap behöver Microsoft .NET {0} Desktop Runtime för att fungera. Vill du installera det nu?\n\nWindows kan be om behörighet under installationen.",
    failed: ".NET {0} Desktop Runtime kunde inte installeras automatiskt. Microsofts nedladdningssida öppnas nu. Installera det därifrån och starta sedan Nullstrap igen.",
    unpack: "Nullstrap kunde inte startas. Kontrollera att det finns ledigt diskutrymme och att ditt antivirusprogram inte blockerar Nullstrap, och försök sedan igen.",
};

const TH: Texts = Texts {
    ask: "Nullstrap ต้องใช้ Microsoft .NET {0} Desktop Runtime ในการทำงาน คุณต้องการติดตั้งตอนนี้หรือไม่\n\nWindows อาจขอสิทธิ์ระหว่างการติดตั้ง",
    failed: "ไม่สามารถติดตั้ง .NET {0} Desktop Runtime โดยอัตโนมัติได้ หน้าดาวน์โหลดของ Microsoft จะเปิดขึ้นตอนนี้ โปรดติดตั้งจากหน้านั้น แล้วเปิด Nullstrap อีกครั้ง",
    unpack: "ไม่สามารถเริ่ม Nullstrap ได้ โปรดตรวจสอบว่าดิสก์มีพื้นที่ว่างและโปรแกรมป้องกันไวรัสของคุณไม่ได้บล็อก Nullstrap แล้วลองอีกครั้ง",
};

const TR: Texts = Texts {
    ask: "Nullstrap'in çalışması için Microsoft .NET {0} Desktop Runtime gerekiyor. Şimdi yüklemek ister misiniz?\n\nWindows yükleme sırasında izin isteyebilir.",
    failed: ".NET {0} Desktop Runtime otomatik olarak yüklenemedi. Microsoft indirme sayfası şimdi açılacak. Oradan yükleyin, ardından Nullstrap'i yeniden başlatın.",
    unpack: "Nullstrap başlatılamadı. Diskte boş alan olduğundan ve virüsten koruma programınızın Nullstrap'i engellemediğinden emin olup tekrar deneyin.",
};

const UK: Texts = Texts {
    ask: "Для роботи Nullstrap потрібен Microsoft .NET {0} Desktop Runtime. Встановити його зараз?\n\nПід час встановлення Windows може запитати дозвіл.",
    failed: "Не вдалося автоматично встановити .NET {0} Desktop Runtime. Зараз відкриється сторінка завантаження Microsoft. Встановіть його звідти, а потім знову запустіть Nullstrap.",
    unpack: "Не вдалося запустити Nullstrap. Переконайтеся, що на диску є вільне місце і Ваш антивірус не блокує Nullstrap, потім спробуйте ще раз.",
};

const VI: Texts = Texts {
    ask: "Nullstrap cần Microsoft .NET {0} Desktop Runtime để hoạt động. Bạn có muốn cài đặt ngay bây giờ không?\n\nWindows có thể yêu cầu cấp quyền trong khi cài đặt.",
    failed: "Không thể tự động cài đặt .NET {0} Desktop Runtime. Trang tải xuống của Microsoft sẽ mở ngay bây giờ. Hãy cài đặt từ đó, sau đó khởi động lại Nullstrap.",
    unpack: "Không thể khởi động Nullstrap. Hãy đảm bảo ổ đĩa còn dung lượng trống và phần mềm diệt virus của bạn không chặn Nullstrap, rồi thử lại.",
};

const ZH_CN: Texts = Texts {
    ask: "Nullstrap 需要 Microsoft .NET {0} Desktop Runtime 才能运行。您要现在安装吗？\n\n安装过程中 Windows 可能会请求权限。",
    failed: "无法自动安装 .NET {0} Desktop Runtime。现在将打开 Microsoft 下载页面。请从该页面安装，然后重新启动 Nullstrap。",
    unpack: "无法启动 Nullstrap。请确认磁盘有可用空间，并且您的杀毒软件没有阻止 Nullstrap，然后重试。",
};

const ZH_TW: Texts = Texts {
    ask: "Nullstrap 需要 Microsoft .NET {0} Desktop Runtime 才能執行。你要現在安裝嗎？\n\n安裝期間 Windows 可能會要求權限。",
    failed: "無法自動安裝 .NET {0} Desktop Runtime。現在將開啟 Microsoft 下載頁面。請從該頁面安裝，然後重新啟動 Nullstrap。",
    unpack: "無法啟動 Nullstrap。請確認磁碟有可用空間，且你的防毒軟體沒有封鎖 Nullstrap，然後再試一次。",
};

fn texts() -> &'static Texts {
    let language = unsafe { GetUserDefaultUILanguage() };
    let primary = language & 0x3ff;
    let sub = language >> 10;
    match primary {
        0x01 => &AR,
        0x02 => &BG,
        0x04 if matches!(sub, 0x01 | 0x03 | 0x05) => &ZH_TW,
        0x04 => &ZH_CN,
        0x05 => &CS,
        0x07 => &DE,
        0x0a => &ES,
        0x0b => &FI,
        0x0c => &FR,
        0x0e => &HU,
        0x10 => &IT,
        0x11 => &JA,
        0x12 => &KO,
        0x13 => &NL,
        0x15 => &PL,
        0x16 => &PT_BR,
        0x18 => &RO,
        0x19 => &RU,
        0x1a if matches!(sub, 0x01 | 0x04) => &HR,
        0x1d => &SV,
        0x1e => &TH,
        0x1f => &TR,
        0x21 => &ID,
        0x22 => &UK,
        0x27 => &LT,
        0x29 => &FA,
        0x2a => &VI,
        0x3e => &MS,
        0x64 => &FIL,
        _ => &EN,
    }
}

fn fill(text: &str) -> String {
    text.replace("{0}", crate::DOTNET_MAJOR)
}

pub fn ask() -> String {
    fill(texts().ask)
}

pub fn failed() -> String {
    fill(texts().failed)
}

pub fn unpack_failed() -> String {
    fill(texts().unpack)
}
