// TODO: Remote test DB doesn't seem to support v2 (handshake fails). Perhaps a Neo4j version issue?

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    use bolt_proto::message::*;
    use bolt_proto::value::*;
    use bolt_proto::{Message, Value};

    use crate::client::v1::tests::*;
    use crate::skip_if_handshake_failed;

    #[tokio::test]
    async fn init() {
        let client = new_client(2).await;
        skip_if_handshake_failed!(client);
        let mut client = client.unwrap();
        let response = initialize_client(&mut client, true).await.unwrap();
        assert!(Success::try_from(response).is_ok());
    }

    #[tokio::test]
    async fn init_fail() {
        let client = new_client(2).await;
        skip_if_handshake_failed!(client);
        let mut client = client.unwrap();
        let response = initialize_client(&mut client, false).await.unwrap();
        assert!(Failure::try_from(response).is_ok());
    }

    // TODO: Test https://github.com/neo4j/neo4j/pull/8050 (ACK_FAILURE on failed INIT)

    #[tokio::test]
    async fn ack_failure() {
        let client = get_initialized_client(2).await;
        skip_if_handshake_failed!(client);
        let mut client = client.unwrap();
        let response = run_invalid_query(&mut client).await.unwrap();
        assert!(Failure::try_from(response).is_ok());
        let response = client.ack_failure().await.unwrap();
        assert!(Success::try_from(response).is_ok());
        let response = run_valid_query(&mut client).await.unwrap();
        assert!(Success::try_from(response).is_ok());
    }

    #[tokio::test]
    async fn ack_failure_after_ignored() {
        let client = get_initialized_client(2).await;
        skip_if_handshake_failed!(client);
        let mut client = client.unwrap();
        let response = run_invalid_query(&mut client).await.unwrap();
        assert!(Failure::try_from(response).is_ok());
        let response = run_valid_query(&mut client).await.unwrap();
        assert!(match response {
            Message::Ignored => true,
            _ => false,
        });
        let response = client.ack_failure().await.unwrap();
        assert!(Success::try_from(response).is_ok());
        let response = run_valid_query(&mut client).await.unwrap();
        assert!(Success::try_from(response).is_ok());
    }

    #[tokio::test]
    async fn run() {
        let client = get_initialized_client(2).await;
        skip_if_handshake_failed!(client);
        let mut client = client.unwrap();
        let response = run_valid_query(&mut client).await.unwrap();
        assert!(Success::try_from(response).is_ok());
    }

    #[tokio::test]
    async fn run_pipelined() {
        let client = get_initialized_client(2).await;
        skip_if_handshake_failed!(client);
        let mut client = client.unwrap();
        let messages = vec![
            Message::Run(Run::new("MATCH (n {test: 'v2-pipelined'}) DETACH DELETE n;".to_string(), Default::default())),
            Message::PullAll,
            Message::Run(Run::new("CREATE (:Database {name: 'neo4j', v1_release: date('2010-02-16'), test: 'v2-pipelined'});".to_string(), Default::default())),
            Message::PullAll,
            Message::Run(Run::new(
                "MATCH (neo4j:Database {name: 'neo4j', test: 'v2-pipelined'}) CREATE (:Library {name: 'bolt-client', v1_release: date('2019-12-23'), test: 'v2-pipelined'})-[:CLIENT_FOR]->(neo4j);".to_string(),
                Default::default())),
            Message::PullAll,
            Message::Run(Run::new(
                "MATCH (neo4j:Database {name: 'neo4j', test: 'v2-pipelined'}), (bolt_client:Library {name: 'bolt-client', test: 'v2-pipelined'}) RETURN duration.between(neo4j.v1_release, bolt_client.v1_release);".to_string(),
                Default::default())),
            Message::PullAll,
        ];
        for response in client.run_pipelined(messages).await.unwrap() {
            assert!(match response {
                Message::Success(_) => true,
                Message::Record(record) => {
                    assert_eq!(
                        Record::try_from(record).unwrap().fields()[0],
                        Value::from(Duration::new(118, 7, 0, 0))
                    );
                    true
                }
                _ => false,
            });
        }
    }

    #[tokio::test]
    async fn run_and_pull() {
        let client = get_initialized_client(2).await;
        skip_if_handshake_failed!(client);
        let mut client = client.unwrap();
        let response = client
            .run(
                "RETURN localdatetime('2010-03-05T12:30:01.000000500');".to_string(),
                None,
            )
            .await
            .unwrap();
        assert!(Success::try_from(response).is_ok());

        let (response, records) = client.pull_all().await.unwrap();
        assert!(Success::try_from(response).is_ok());
        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0].fields(),
            &[Value::from(NaiveDateTime::new(
                NaiveDate::from_ymd(2010, 3, 5),
                NaiveTime::from_hms_nano(12, 30, 1, 500)
            ))]
        );

        let response = client
            .run(
                "RETURN point({x: 42.5123, y: 1.123, z: 3214});".to_string(),
                None,
            )
            .await
            .unwrap();
        assert!(Success::try_from(response).is_ok());

        let (response, records) = client.pull_all().await.unwrap();
        assert!(Success::try_from(response).is_ok());
        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0].fields(),
            &[Value::from(Point3D::new(9157, 42.5123, 1.123, 3214.0))]
        );
    }

    #[tokio::test]
    async fn node_and_rel_creation() {
        let client = get_initialized_client(2).await;
        skip_if_handshake_failed!(client);
        let mut client = client.unwrap();
        let statement = "MATCH (n {test: 'v2-node-rel'}) DETACH DELETE n;".to_string();
        client.run(statement, None).await.unwrap();
        client.pull_all().await.unwrap();

        let statement =
            "CREATE (:Client {name: 'bolt-client', starting: datetime('2019-12-19T16:08:04-08:00'), test: 'v2-node-rel'})-[:WRITTEN_IN]->(:Language {name: 'Rust', test: 'v2-node-rel'});"
                .to_string();
        client.run(statement, None).await.unwrap();
        client.pull_all().await.unwrap();
        let statement =
            "MATCH (c {test: 'v2-node-rel'})-[r:WRITTEN_IN]->(l) RETURN c, r, l;".to_string();
        client.run(statement, None).await.unwrap();
        let (_response, records) = client.pull_all().await.unwrap();

        let c = Node::try_from(records[0].fields()[0].clone()).unwrap();
        let r = Relationship::try_from(records[0].fields()[1].clone()).unwrap();
        let l = Node::try_from(records[0].fields()[2].clone()).unwrap();

        assert_eq!(c.labels(), &["Client".to_string()]);
        assert_eq!(
            c.properties().get("name"),
            Some(&Value::from("bolt-client"))
        );
        assert_eq!(
            c.properties().get("starting"),
            Some(&Value::from(
                DateTimeOffset::new(2019, 12, 19, 16, 8, 4, 0, (-8, 0)).unwrap()
            ))
        );
        assert_eq!(l.labels(), &["Language".to_string()]);
        assert_eq!(l.properties().get("name"), Some(&Value::from("Rust")));
        assert_eq!(r.rel_type(), "WRITTEN_IN");
        assert!(r.properties().is_empty());
        assert_eq!(
            (r.start_node_identity(), r.end_node_identity()),
            (c.node_identity(), l.node_identity())
        );
    }

    #[tokio::test]
    async fn discard_all_fail() {
        let client = get_initialized_client(2).await;
        skip_if_handshake_failed!(client);
        let mut client = client.unwrap();
        let response = client.discard_all().await.unwrap();
        assert!(Failure::try_from(response).is_ok());
    }

    #[tokio::test]
    async fn discard_all() {
        let client = get_initialized_client(2).await;
        skip_if_handshake_failed!(client);
        let mut client = client.unwrap();
        let response = run_valid_query(&mut client).await.unwrap();
        assert!(Success::try_from(response).is_ok());
        let response = client.discard_all().await.unwrap();
        assert!(Success::try_from(response).is_ok());
    }

    #[tokio::test]
    async fn discard_all_and_pull() {
        let client = get_initialized_client(2).await;
        skip_if_handshake_failed!(client);
        let mut client = client.unwrap();
        let response = run_valid_query(&mut client).await.unwrap();
        assert!(Success::try_from(response).is_ok());
        let response = client.discard_all().await.unwrap();
        assert!(Success::try_from(response).is_ok());
        let (response, records) = client.pull_all().await.unwrap();
        assert!(Failure::try_from(response).is_ok());
        assert!(records.is_empty());
    }

    #[tokio::test]
    async fn reset() {
        let client = get_initialized_client(2).await;
        skip_if_handshake_failed!(client);
        let mut client = client.unwrap();
        let response = run_invalid_query(&mut client).await.unwrap();
        assert!(Failure::try_from(response).is_ok());
        let response = run_valid_query(&mut client).await.unwrap();
        assert!(match response {
            Message::Ignored => true,
            _ => false,
        });
        let response = client.reset().await.unwrap();
        assert!(Success::try_from(response).is_ok());
        let response = run_valid_query(&mut client).await.unwrap();
        assert!(Success::try_from(response).is_ok());
    }

    #[tokio::test]
    async fn ignored() {
        let client = get_initialized_client(2).await;
        skip_if_handshake_failed!(client);
        let mut client = client.unwrap();
        let response = run_invalid_query(&mut client).await.unwrap();
        assert!(Failure::try_from(response).is_ok());
        let response = run_valid_query(&mut client).await.unwrap();
        assert!(match response {
            Message::Ignored => true,
            _ => false,
        });
    }
}
