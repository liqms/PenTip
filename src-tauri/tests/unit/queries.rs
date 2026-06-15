use pentip_lib::db::queries::{attach_tag, create_fragment, create_page, create_project, create_tag, delete_fragment, delete_page, delete_project, delete_tag, detach_tag, get_fragment, list_fragments, list_pages, list_projects, list_tags, list_tags_for_target, update_fragment, update_page, update_project};

use crate::common::setup_db;

// ─── Fragment Tests ──────────────────────────

#[tokio::test]
async fn test_create_fragment() {
    let db = setup_db().await;
    let now = 1000000;
    let fragment = create_fragment(&db, "frag-1", "Hello, world!", "idea", None, now, now)
        .await
        .unwrap();
    assert_eq!(fragment.id, "frag-1");
    assert_eq!(fragment.content, "Hello, world!");
    assert_eq!(fragment.note_type, "idea");
    assert!(fragment.source.is_none());
    assert_eq!(fragment.created_at, now);
    assert_eq!(fragment.updated_at, now);
    assert!(fragment.deleted_at.is_none());
}

#[tokio::test]
async fn test_list_fragments_empty() {
    let db = setup_db().await;
    let fragments = list_fragments(&db).await.unwrap();
    assert!(fragments.is_empty());
}

#[tokio::test]
async fn test_list_fragments_returns_all() {
    let db = setup_db().await;
    let now = 1000000;
    create_fragment(&db, "f1", "First", "idea", None, now, now).await.unwrap();
    create_fragment(&db, "f2", "Second", "todo", None, now + 1, now + 1).await.unwrap();
    let fragments = list_fragments(&db).await.unwrap();
    assert_eq!(fragments.len(), 2);
    // Ordered by created_at DESC
    assert_eq!(fragments[0].id, "f2");
    assert_eq!(fragments[1].id, "f1");
}

#[tokio::test]
async fn test_get_fragment() {
    let db = setup_db().await;
    let now = 1000000;
    create_fragment(&db, "f1", "Content", "memo", None, now, now).await.unwrap();
    let fragment = get_fragment(&db, "f1").await.unwrap();
    assert_eq!(fragment.content, "Content");
}

#[tokio::test]
async fn test_get_fragment_not_found() {
    let db = setup_db().await;
    let result = get_fragment(&db, "nonexistent").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_fragment() {
    let db = setup_db().await;
    let now = 1000000;
    create_fragment(&db, "f1", "Original", "idea", None, now, now).await.unwrap();
    let updated = update_fragment(&db, "f1", "Updated", now + 1).await.unwrap();
    assert_eq!(updated.content, "Updated");
    assert_eq!(updated.updated_at, now + 1);
}

#[tokio::test]
async fn test_delete_fragment_soft_delete() {
    let db = setup_db().await;
    let now = 1000000;
    create_fragment(&db, "f1", "To delete", "idea", None, now, now).await.unwrap();
    delete_fragment(&db, "f1", now + 1).await.unwrap();
    let fragments = list_fragments(&db).await.unwrap();
    assert!(fragments.is_empty());
    let result = get_fragment(&db, "f1").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_fragment_with_source() {
    let db = setup_db().await;
    let now = 1000000;
    let fragment = create_fragment(&db, "f1", "With source", "diary", Some("Twitter"), now, now)
        .await
        .unwrap();
    assert_eq!(fragment.source, Some("Twitter".to_string()));
}

// ─── Tag Tests ───────────────────────────────

#[tokio::test]
async fn test_create_tag() {
    let db = setup_db().await;
    let now = 1000000;
    let tag = create_tag(&db, "tag-1", "rust", now).await.unwrap();
    assert_eq!(tag.id, "tag-1");
    assert_eq!(tag.name, "rust");
}

#[tokio::test]
async fn test_list_tags() {
    let db = setup_db().await;
    let now = 1000000;
    create_tag(&db, "t1", "alpha", now).await.unwrap();
    create_tag(&db, "t2", "beta", now).await.unwrap();
    let tags = list_tags(&db).await.unwrap();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0].name, "alpha");
    assert_eq!(tags[1].name, "beta");
}

#[tokio::test]
async fn test_delete_tag() {
    let db = setup_db().await;
    let now = 1000000;
    create_tag(&db, "t1", "delete-me", now).await.unwrap();
    delete_tag(&db, "t1").await.unwrap();
    let tags = list_tags(&db).await.unwrap();
    assert!(tags.is_empty());
}

#[tokio::test]
async fn test_attach_and_list_tags_for_target() {
    let db = setup_db().await;
    let now = 1000000;
    create_tag(&db, "t1", "feature", now).await.unwrap();
    create_tag(&db, "t2", "bug", now).await.unwrap();
    attach_tag(&db, "t1", "fragment", "frag-1", now).await.unwrap();
    attach_tag(&db, "t2", "fragment", "frag-1", now).await.unwrap();
    let tags = list_tags_for_target(&db, "fragment", "frag-1").await.unwrap();
    assert_eq!(tags.len(), 2);
}

#[tokio::test]
async fn test_detach_tag() {
    let db = setup_db().await;
    let now = 1000000;
    create_tag(&db, "t1", "temp", now).await.unwrap();
    attach_tag(&db, "t1", "fragment", "frag-1", now).await.unwrap();
    detach_tag(&db, "t1", "fragment", "frag-1").await.unwrap();
    let tags = list_tags_for_target(&db, "fragment", "frag-1").await.unwrap();
    assert!(tags.is_empty());
}

// ─── Project Tests ───────────────────────────

#[tokio::test]
async fn test_create_project() {
    let db = setup_db().await;
    let now = 1000000;
    let project = create_project(&db, "proj-1", "My Novel", "novel", now, now)
        .await
        .unwrap();
    assert_eq!(project.id, "proj-1");
    assert_eq!(project.title, "My Novel");
    assert_eq!(project.project_type, "novel");
    assert_eq!(project.status, "in-progress");
}

#[tokio::test]
async fn test_list_projects() {
    let db = setup_db().await;
    let now = 1000000;
    create_project(&db, "p1", "First", "article", now, now).await.unwrap();
    create_project(&db, "p2", "Second", "novel", now, now + 1).await.unwrap();
    let projects = list_projects(&db).await.unwrap();
    assert_eq!(projects.len(), 2);
    assert_eq!(projects[0].id, "p2");
    assert_eq!(projects[1].id, "p1");
}

#[tokio::test]
async fn test_update_project() {
    let db = setup_db().await;
    let now = 1000000;
    create_project(&db, "p1", "Original", "article", now, now).await.unwrap();
    let updated = update_project(&db, "p1", "Renamed", now + 1).await.unwrap();
    assert_eq!(updated.title, "Renamed");
}

#[tokio::test]
async fn test_delete_project_soft_delete() {
    let db = setup_db().await;
    let now = 1000000;
    create_project(&db, "p1", "To delete", "article", now, now).await.unwrap();
    delete_project(&db, "p1", now + 1).await.unwrap();
    let projects = list_projects(&db).await.unwrap();
    assert!(projects.is_empty());
}

// ─── Page Tests ──────────────────────────────

#[tokio::test]
async fn test_create_page() {
    let db = setup_db().await;
    let now = 1000000;
    create_project(&db, "proj-1", "Project", "article", now, now).await.unwrap();
    let page = create_page(&db, "page-1", "proj-1", "Chapter 1", 1, now, now)
        .await
        .unwrap();
    assert_eq!(page.id, "page-1");
    assert_eq!(page.title, "Chapter 1");
    assert_eq!(page.sort_order, 1);
}

#[tokio::test]
async fn test_list_pages() {
    let db = setup_db().await;
    let now = 1000000;
    create_project(&db, "proj-1", "Project", "article", now, now).await.unwrap();
    create_page(&db, "p1", "proj-1", "Second", 2, now, now).await.unwrap();
    create_page(&db, "p2", "proj-1", "First", 1, now, now).await.unwrap();
    let pages = list_pages(&db, "proj-1").await.unwrap();
    assert_eq!(pages.len(), 2);
    assert_eq!(pages[0].title, "First");
    assert_eq!(pages[1].title, "Second");
}

#[tokio::test]
async fn test_update_page() {
    let db = setup_db().await;
    let now = 1000000;
    create_project(&db, "proj-1", "Project", "article", now, now).await.unwrap();
    create_page(&db, "p1", "proj-1", "Draft", 1, now, now).await.unwrap();
    let updated = update_page(&db, "p1", "Final", "Full content here", now + 1).await.unwrap();
    assert_eq!(updated.title, "Final");
    assert_eq!(updated.content, Some("Full content here".to_string()));
}

#[tokio::test]
async fn test_delete_page_soft_delete() {
    let db = setup_db().await;
    let now = 1000000;
    create_project(&db, "proj-1", "Project", "article", now, now).await.unwrap();
    create_page(&db, "p1", "proj-1", "To delete", 1, now, now).await.unwrap();
    delete_page(&db, "p1", now + 1).await.unwrap();
    let pages = list_pages(&db, "proj-1").await.unwrap();
    assert!(pages.is_empty());
}