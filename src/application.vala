/* application.vala
 *
 * Copyright 2022 Bnyro
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

namespace GtkRest {
    public class Application : Adw.Application {
        public Application () {
            Object (
                application_id: "com.bnyro.rest",
                flags: ApplicationFlags.FLAGS_NONE
            );
        }

        private Soup.Session session;
        private string url;

        construct {
            ActionEntry[] action_entries = {
                { "about", this.on_about_action },
                { "preferences", this.on_preferences_action },
                { "quit", this.quit }
            };
            this.add_action_entries (action_entries, this);
            this.set_accels_for_action ("app.quit", { "<primary>q" });
        }

        public override void activate () {
            base.activate ();
            var win = this.active_window;
            if (win == null) {
                win = new GtkRest.Window (this);
            }
            setup_window ((GtkRest.Window) win);
            create_session ();
            win.present ();
        }

        private void on_about_action () {
            var about = new Widget.AboutDialog ();
            about.dialog.transient_for = this.active_window;
            about.dialog.present ();
        }

        private void on_preferences_action () {
            message ("app.preferences action activated");
        }

        private void setup_window (GtkRest.Window window) {
            window.send.clicked.connect (() => {
                start_request ();
            });
            url = window.url.text;
            window.url.changed.connect (() => {
                this.url = window.url.text;
            });
        }

        private void create_session () {
            this.session = new Soup.Session ();
            this.session.user_agent = "Gtk Rest/1.0";
            this.session.timeout = 20;
        }

        private void start_request () {
            var message = new Soup.Message ("GET", url);
            this.session.queue_message (message, (ses, msg) => {
                var body = (string) msg.response_body.flatten ().data;
                var window = (GtkRest.Window) this.active_window;
                window.response.label = body;
            });
        }
    }
}
