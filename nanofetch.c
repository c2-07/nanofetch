#include <stdio.h>
#include <stdlib.h>

int main(void) {
    // Cache once
    const char *user  = getenv("USER");
    const char *term  = getenv("TERM");
    const char *shell = getenv("SHELL");

    // Fallbacks
    if (!user)  user  = "(unknown)";
    if (!term)  term  = "(unknown)";
    if (!shell) shell = "(unknown)";

    // ANSI escape sequences precomputed
    const char *cyan_bold = "\033[1;96m";
    const char *reset     = "\033[0m";

    // Print in one shot to reduce I/O overhead
    fputs("\n", stdout);

    fputs(cyan_bold, stdout);  fputs("  user", stdout);  fputs(reset, stdout);  fputs("  ", stdout);  fputs(user, stdout);  fputc('\n', stdout);
    fputs(cyan_bold, stdout);  fputs("  term", stdout);  fputs(reset, stdout);  fputs("  ", stdout);  fputs(term, stdout);  fputc('\n', stdout);
    fputs(cyan_bold, stdout);  fputs("  shell", stdout); fputs(reset, stdout);  fputs("  ", stdout);  fputs(shell, stdout); fputc('\n', stdout);

    fputs("\n\n", stdout);

    return 0;
}
