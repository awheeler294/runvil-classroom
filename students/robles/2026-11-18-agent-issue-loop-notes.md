# Notes: one issue-to-PR loop with a cloud agent on runvil-gateway

One trial, run by hand on 2026-11-13. Issue: https://github.com/cppalliance/runvil-gateway/issues/10. Result: https://github.com/cppalliance/runvil-gateway/pull/12 (approved by Andrew, not landed).

## What I did

1. Wrote issue #10 so an agent could act on it without questions. It states what is wrong, the test that should fail before the fix and pass after, and what is out of scope.
2. Added an `agent-ready` label. The label is the gate between "reported" and "an agent may change code".
3. Copied the issue into my fork and commented `@cursor fix this issue on master in this fork. One commit. ...` on the copy. A Cursor cloud agent started in about 10 seconds and pushed a branch with one commit.
4. Checked the fix locally, then opened it upstream by hand as #12.

## What it needed

- An issue that carries the test and the scope. The trigger comment could be one line because the issue held the detail. In your version the `plan.md` plays this part.
- A GitHub account linked to the agent service. I used my own account.
- Somewhere the agent may push. I used my fork, because topical branches are not allowed in the main repositories.
- A rule for who may start it. Mine: the label must have been added by someone with Triage access or above.

## What is not built

- Automatic pickup. I have a script that turns the label into the fork issue and the trigger comment. It has only run in dry-run mode.
- Opening the upstream pull request from the agent's branch. I did that by hand.
- A checkbox trigger and a Claude route. I used a label and Cursor, and have not tried either of the others.

## Things to watch

- Commit identity. In my run the commit author was Cursor Agent, with my account as co-author. Check author and committer on every commit before it goes upstream.
- Cost. Each run is billed to the linked account, so anyone who can trigger it can spend it.
- Reruns. Without a record of what was already dispatched, the same issue gets picked up twice. My script treats an existing fork copy as "already dispatched".
