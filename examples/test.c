#include <stdio.h>
#include <stdlib.h>
#include <libproc.h>
#include <sys/proc_info.h>
#include <err.h>
#include <stdint.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <errno.h>
#include <string.h>

static struct proc_fileportinfo *fps = (struct proc_fileportinfo *)NULL;
static int nbfps = 0;

void process_fileport_vnode(int pid, uint32_t fp) {
    int nb;
    char access = ' ';
    struct vnode_fdinfowithpath vi;

    nb = proc_pidfileportinfo(pid, fp, PROC_PIDFILEPORTVNODEPATHINFO, &vi, sizeof(vi));
    if (nb < sizeof(vi))
        err(1, "pidfileportinfo\n");

    // enter file info
    struct proc_fileinfo *pfi = &vi.pfi;
    int f = pfi->fi_openflags & (FREAD | FWRITE);
    if (f == FREAD)
        access = 'r';
    else if (f == FWRITE)
        access = 'w';
    else if (f == (FREAD | FWRITE))
        access = 'u';

    // enter vnode info
    struct vnode_info_path *vip = &vi.pvip;
    printf("path: %s\ncap: %c\n", vip->vip_path, access);
}

int main(int argc, char **argv) {
    if (argc < 2) {
        int nbpids = proc_listpids(PROC_ALL_PIDS, 0, NULL, 0);
        printf("pids = %d\n", (int)(nbpids / (sizeof (int))));
        if (nbpids <= 0)
            err(1, "can't get nbpids\n");
        int *pids = malloc(nbpids);
        if (!pids)
            err(1, "malloc\n");
        if (proc_listpids(PROC_ALL_PIDS, 0, pids, nbpids) <= 0)
            err(1, "Can't get pids\n");

        for (int i = 0; i < nbpids; i++) {
            int pid = pids[i];
            if (pid == 0)
                continue;
            //int nbfps = 1024;
            int nb = proc_pidinfo(pid, PROC_PIDLISTFILEPORTS, 0, fps, nbfps);
            if (nb == 0) {
//                if (errno == EPERM)
//                    continue;
//                if (errno == 3)
//                    continue;
                fprintf(stderr, "pid: %d\nCan't get fileport byte count: %s(%d)\n", pid, strerror(errno), errno);
                continue;
            }
            if (nb < 0)
                err(1, "can't get nbfps");
            if (nb < sizeof(struct proc_fileportinfo))
                err(1, "fps size error");
            fps = (struct proc_fileportinfo *)malloc(nbfps);
            if (fps == NULL)
                err(1, "malloc");
            if ((nb = proc_pidinfo(pid, PROC_PIDLISTFILEPORTS, 0, fps, nbfps)) < 0)
                err(1, "can't get fileports");

            int nf = (int)(nb / sizeof(struct proc_fileportinfo));
            for (int j = 0; j < nf; j++) {
                struct proc_fileportinfo fpi = fps[j];
                char *fdtype;
                switch (fpi.proc_fdtype) {
                    case PROX_FDTYPE_PIPE:
                        fdtype = "PIPE";
                        break;
                    case PROX_FDTYPE_PSEM:
                        fdtype = "PSEM";
                        break;
                    case PROX_FDTYPE_PSHM:
                        fdtype = "PSHM";
                        break;
                    case PROX_FDTYPE_ATALK:
                        fdtype = "ATALK";
                        break;
                    case PROX_FDTYPE_NEXUS:
                        fdtype = "NEXUS";
                        break;
                    case PROX_FDTYPE_VNODE:
                        fdtype = "VNODE";
                        break;
                    case PROX_FDTYPE_KQUEUE:
                        fdtype = "KQUEUE";
                        break;
                    case PROX_FDTYPE_SOCKET:
                        fdtype = "SOCKET";
                        break;
                    case PROX_FDTYPE_CHANNEL:
                        fdtype = "CHANNEL";
                        break;
                    case PROX_FDTYPE_FSEVENTS:
                        fdtype = "FDEVENTS";
                        break;
                    case PROX_FDTYPE_NETPOLICY:
                        fdtype = "NETPOLICY";
                        break;
                    default:
                        fdtype = "Unknown";
                        break;
                }
                printf("fileport: %d\nproc_fdtype: %s\n", fpi.proc_fileport, fdtype);

                if (fpi.proc_fdtype == PROX_FDTYPE_VNODE) {
                    process_fileport_vnode(pid, fpi.proc_fileport);
                }
            }
        }
    } else {
        int pid = atoi(argv[1]);
        int nbfps = proc_pidinfo(pid, PROC_PIDLISTFILEPORTS, 0, NULL, 0);
        if (nbfps == 0) {
            fprintf(stderr, "Can't get fileport byte count: %s\n", strerror(errno));
            return 1;
        }
        if (nbfps < 0)
            err(1, "can't get nbfps");
        struct proc_fileportinfo *fps = (struct proc_fileportinfo *)malloc(nbfps);
        if (!fps)
            err(1, "malloc");
        int nb;
        if ((nb = proc_pidinfo(pid, PROC_PIDLISTFILEPORTS, 0, fps, nbfps)) < 0)
            err(1, "can't get fileports");

        int nf = (int)(nb / sizeof(struct proc_fileportinfo));
        for (int j = 0; j < nf; j++) {
            struct proc_fileportinfo fpi = fps[j];
            char *fdtype;
            switch (fpi.proc_fdtype) {
                case PROX_FDTYPE_PIPE:
                    fdtype = "PIPE";
                    break;
                case PROX_FDTYPE_PSEM:
                    fdtype = "PSEM";
                    break;
                case PROX_FDTYPE_PSHM:
                    fdtype = "PSHM";
                    break;
                case PROX_FDTYPE_ATALK:
                    fdtype = "ATALK";
                    break;
                case PROX_FDTYPE_NEXUS:
                    fdtype = "NEXUS";
                    break;
                case PROX_FDTYPE_VNODE:
                    fdtype = "VNODE";
                    break;
                case PROX_FDTYPE_KQUEUE:
                    fdtype = "KQUEUE";
                    break;
                case PROX_FDTYPE_SOCKET:
                    fdtype = "SOCKET";
                    break;
                case PROX_FDTYPE_CHANNEL:
                    fdtype = "CHANNEL";
                    break;
                case PROX_FDTYPE_FSEVENTS:
                    fdtype = "FDEVENTS";
                    break;
                case PROX_FDTYPE_NETPOLICY:
                    fdtype = "NETPOLICY";
                    break;
                default:
                    fdtype = "Unknown";
                    break;
            }
            printf("fileport: %d\nproc_fdtype: %s\n", fpi.proc_fileport, fdtype);

            if (fpi.proc_fdtype == PROX_FDTYPE_VNODE) {
                process_fileport_vnode(pid, fpi.proc_fileport);
            }
        }
    }
}

