import unittest

import dispatch as d

URL10 = "https://github.com/cppalliance/runvil-gateway/issues/10"
URL1 = "https://github.com/cppalliance/runvil-gateway/issues/1"


class BodyFormatting(unittest.TestCase):
    def test_marker_blank_line_then_body_unchanged(self):
        body = "### What happened?\n\nx `y`\n"
        out = d.format_fork_body(URL10, body)
        lines = out.split("\n")
        self.assertEqual(
            lines[0],
            f"Copy of {URL10} for a Cursor cloud agent. Fix it here, in this fork.")
        self.assertEqual(lines[1], "")
        self.assertEqual(out.split("\n\n", 1)[1], body)

    def test_none_body(self):
        self.assertEqual(d.format_fork_body(URL10, None).split("\n\n", 1)[1], "")


class Idempotence(unittest.TestCase):
    up = {"number": 10, "url": URL10, "title": "T"}

    def test_marker_match(self):
        fi = {"number": 5, "title": "other", "body": d.format_fork_body(URL10, "b")}
        self.assertEqual(d.find_fork_issue(self.up, [fi])[1], "marker")

    def test_prefix_url_does_not_match(self):
        fi = {"number": 5, "title": "other", "body": d.format_fork_body(URL1, "b")}
        self.assertIsNone(d.find_fork_issue(self.up, [fi]))

    def test_url_only_in_later_line_does_not_match(self):
        fi = {"number": 5, "title": "other", "body": "hello\n" + URL10}
        self.assertIsNone(d.find_fork_issue(self.up, [fi]))

    def test_marker_for_other_issue_same_title_not_legacy(self):
        fi = {"number": 5, "title": "T", "body": d.format_fork_body(URL1, "b")}
        self.assertIsNone(d.find_fork_issue(self.up, [fi]))

    def test_legacy_title_match_for_hand_made_copy(self):
        fi = {"number": 2, "title": "T", "body": "What happened?\nx"}
        self.assertEqual(d.find_fork_issue(self.up, [fi])[1], "legacy title match")

    def test_legacy_match_ignores_backticks(self):
        up = {"number": 10, "url": URL10, "title": "Empty `[server] api_key` is bad"}
        fi = {"number": 2, "title": "Empty [server] api_key is bad", "body": "What happened?"}
        self.assertEqual(d.find_fork_issue(up, [fi])[1], "legacy title match")

    def test_empty_fork(self):
        self.assertIsNone(d.find_fork_issue(self.up, []))

    def test_none_body(self):
        self.assertIsNone(d.find_fork_issue(self.up, [{"number": 1, "title": "z", "body": None}]))


class Trust(unittest.TestCase):
    def test_roles(self):
        self.assertTrue(d.role_is_trusted({"role_name": "triage", "permission": "read"}))
        self.assertTrue(d.role_is_trusted({"role_name": "maintain", "permission": "write"}))
        self.assertFalse(d.role_is_trusted({"role_name": "read", "permission": "read"}))
        self.assertFalse(d.role_is_trusted({"permission": "none"}))
        self.assertFalse(d.role_is_trusted(None))

    def test_latest_labeler(self):
        ev = [
            {"event": "labeled", "label": {"name": "agent-ready"}, "actor": {"login": "a"}},
            {"event": "labeled", "label": {"name": "bug"}, "actor": {"login": "z"}},
            {"event": "labeled", "label": {"name": "agent-ready"}, "actor": {"login": "b"}},
        ]
        self.assertEqual(d.latest_labeler(ev, "agent-ready"), "b")
        self.assertIsNone(d.latest_labeler([], "agent-ready"))


if __name__ == "__main__":
    unittest.main()
