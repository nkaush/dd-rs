/**
 * deepfried_dd
 * CS 241 - Spring 2022
 */
#include "format.h"

#include <stdlib.h>
#include <unistd.h>
#include <signal.h>
#include <stdio.h>
#include <errno.h>
#include <time.h>

#define NS_PER_SECOND 1000000000

extern char *optarg;
extern int optind, opterr, optopt;

static int print_status = 0;

typedef struct _config {
    FILE* in;
    FILE* out;
    size_t block_size;
    ssize_t blocks_copied;
    size_t input_blocks_skipped;
    size_t output_blocks_skipped;
} config_t;

config_t* parse_command_line_args(int argc, char* argv[]) {
    config_t* cfg = malloc(sizeof(config_t));
    cfg->in = stdin;
    cfg->out = stdout;
    cfg->block_size = 512;
    cfg->blocks_copied = -1; // if -1, copy entire file
    cfg->input_blocks_skipped = 0;
    cfg->output_blocks_skipped = 0;

    int opt = 0;
    opterr = 0;

    // adapted from https://www.gnu.org/software/libc/manual/html_node/Example-of-Getopt.html
    while ((opt = getopt(argc, argv, "i:o:b:c:p:k:")) != -1) {
        switch (opt) {
            case 'i':
                cfg->in = fopen(optarg, "r");
                if (cfg->in == NULL) { 
                    print_invalid_input(optarg);
                    free(cfg); 
                    exit(1); 
                }

                break;
            case 'o':
                cfg->out = fopen(optarg, "r+");
                if (cfg->out == NULL) { 
                    cfg->out = fopen(optarg, "w");

                    if (cfg->out == NULL) {
                        print_invalid_output(optarg);
                        free(cfg); 
                        exit(1);
                    }
                }

                break;
            case 'b':
                cfg->block_size = strtoul(optarg, NULL, 10);
                break;
            case 'c':
                cfg->blocks_copied = strtoul(optarg, NULL, 10);
                break;
            case 'p':
                cfg->input_blocks_skipped = strtoul(optarg, NULL, 10);
                break;
            case 'k':
                cfg->output_blocks_skipped = strtoul(optarg, NULL, 10);
                break;
            default:
                free(cfg);
                exit(1);
        }
    }

    return cfg;
}

ssize_t write_all(int fd, const char *buffer, size_t count) {
    ssize_t return_code = 0;
    ssize_t bytes_sent = 0;
    
    while (count > 0) {
        return_code = write(fd, buffer + bytes_sent, count);

        if (return_code == 0) {
            return bytes_sent;
        } else if (return_code > 0) {
            bytes_sent += return_code;
            count -= return_code;
        } else if (return_code == -1 && errno == EINTR) {
            continue;
        } else {
            return -1;
        }
    }

    return bytes_sent;
}

ssize_t read_all(int fd, char *buffer, size_t count) {
    ssize_t return_code = 0;
    size_t bytes_read = 0;

    while ( count > 0 ) {
        return_code = read(fd, buffer + bytes_read, count);

        if (return_code == 0) {
            return bytes_read;
        } else if (return_code > 0) {
            bytes_read += return_code;
            count -= return_code;
        } else if (return_code == -1 && errno == EINTR) {
            continue;
        } else {
            return -1;
        }
    }

    return bytes_read;
}

void handle_sigusr1(int signal) { print_status = 1; }

double timespec_difftime(struct timespec* start, struct timespec* finish) {
    double delta = (finish->tv_sec - start->tv_sec) * NS_PER_SECOND;
    delta += (double) (finish->tv_nsec - start->tv_nsec);
    double duration = delta / NS_PER_SECOND;

    return duration;
}

int continue_copying(config_t* cfg, ssize_t num_bytes_copied) {
    if (cfg->blocks_copied == -1) {
        return !feof(cfg->in);
    } else {
        return cfg->blocks_copied * (ssize_t) cfg->block_size != num_bytes_copied && !feof(cfg->in);
    }
}

int main(int argc, char **argv) {
    config_t* cfg = parse_command_line_args(argc, argv);
    signal(SIGUSR1, handle_sigusr1);

    size_t total_bytes_copied = 0;
    struct timespec start_time, curr_time;
    
    clock_gettime(CLOCK_REALTIME, &start_time);

    fseek(cfg->in, cfg->input_blocks_skipped * cfg->block_size, SEEK_SET);
    fseek(cfg->out, cfg->output_blocks_skipped * cfg->block_size, SEEK_SET);

    char* buffer = malloc(cfg->block_size);

    while ( continue_copying(cfg, total_bytes_copied) ) {
        if ( print_status ) {
            print_status = 0;
            clock_gettime(CLOCK_REALTIME, &curr_time);

            print_status_report(
                total_bytes_copied / cfg->block_size, 1,
                total_bytes_copied / cfg->block_size, 1, 
                total_bytes_copied, timespec_difftime(&start_time, &curr_time)
            );
        }

        ssize_t bytes_read = read_all(fileno(cfg->in), buffer, cfg->block_size);

        if (bytes_read == 0) {
            break;
        }

        ssize_t written = write_all(fileno(cfg->out), buffer, bytes_read);

        total_bytes_copied += written;
    }

    clock_gettime(CLOCK_REALTIME, &curr_time);
    print_status_report(
        total_bytes_copied / cfg->block_size, 
        total_bytes_copied % cfg->block_size != 0,
        total_bytes_copied / cfg->block_size, 
        total_bytes_copied % cfg->block_size != 0,
        total_bytes_copied, timespec_difftime(&start_time, &curr_time)
    );
    
    free(buffer);
    free(cfg);
    return 0;
}